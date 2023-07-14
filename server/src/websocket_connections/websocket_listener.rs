use crate::{websocket_connections::{connection::Connection, ForceLock}, listener::Listener, log};
use tokio_tungstenite::tungstenite::Message;
use std::{net::SocketAddr, sync::{Arc, Mutex}, pin::pin};

use futures_util::{future::{self, Either}, StreamExt, SinkExt};

use tokio::sync::{mpsc, broadcast};
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(address: &str) {
    // Create the event loop and TCP listener we'll accept connections on.
    let tcp_listener = TcpListener::bind(&address).await.unwrap_or_else(|err| {
        panic!("Failed to bind websocket server to address {address}: {err}")
    });
    
    // Used to terminate connection threads and stop server
    let mut crash_signal = broadcast::channel(1);

    // Set the panic hook to stop the server gracefully
    {
        // Remove the hook from the previous server instance, if any.
        let _ = std::panic::take_hook();
        // Set the new hook
        let panic_crash_signal_sender = crash_signal.0.clone();
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = panic_crash_signal_sender.send(());
            original_hook(info)
        }))
    }

    let event_listener = Arc::new(Mutex::new(Listener::new()));
    Listener::start(event_listener.clone());

    log!(important "Server"; "Started listening on {address}");

    loop {
        let (stream, addr) = match future::select(
            pin!(tcp_listener.accept()), 
            pin!(crash_signal.1.recv())
        ).await {
            Either::Left((Ok((stream, addr)), _)) => (stream, addr),
            Either::Left((Err(_), _)) => continue, // tcp connection failed, carry on.
            Either::Right(_) => break // crash signal recieved, stop server.
        };
        
        let event_listener = event_listener.clone();
        let crash_signal = (crash_signal.0.clone(), crash_signal.1.resubscribe());

        tokio::spawn(async move {
            // Handle connection runs until the connection or server is closed
            if let Ok(connection) = handle_connection(stream, addr, event_listener.clone(), crash_signal).await {
                match event_listener.force_lock().on_disconnect(connection) {
                    Ok(()) => log!(important "Connection"; "Disconnected {}", addr),
                    Err(reason) => log!(error "Connection"; "Failed to disconnect {}: {}", addr, reason)
                };
            } 
        });
    }

    log!(fatal "Server"; "The server panicked!");
    log!(important "Server"; "Shutting down...");
}

struct ConnectionError;

// Code within this function __SHOULD NOT PANIC__ except for listener methods.
// There is a panic hook that restarts the server. The server doesn't need to restart if a connection fails, so don't panic -- just disconnect.
/// This runs until the connection is closed. It does not remove the connection from the listener.
async fn handle_connection(
    raw_stream: TcpStream, 
    addr: SocketAddr, 
    listener: Arc<Mutex<Listener>>,
    mut crash_signal: (broadcast::Sender<()>, broadcast::Receiver<()>)
) -> Result<Connection, ConnectionError> {
    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(ws_stream) => ws_stream,
        Err(error) => {
            log!(error "Connection"; "Failed to accept websocket handshake with {}: {}", addr, error);
            return Err(ConnectionError);
        }
    };

    // Sending to client MPSC (This gets recieved and rerouted to the client over TCP)
    let (mpsc_sender, mut mpsc_reciever) = mpsc::unbounded_channel();

    // Client TCP connection
    let (mut tcp_sender, mut tcp_reciever) = ws_stream.split();
    
    let connection = {
        let Ok(mut listener) = listener.lock() else {
            let _ = crash_signal.0.send(());
            let _ = tcp_sender.close().await;
            return Err(ConnectionError)
        };
        let connection = Connection::new(mpsc_sender, addr);
        log!(important "Connection"; "Connected: {}", addr);
        listener.on_connect(&connection);
        connection
    };
    
    // Route MPSC packets to client via TCP
    let send_over_tcp = tokio::spawn(async move {
        loop {
            let message = match future::select(pin!(mpsc_reciever.recv()), pin!(crash_signal.1.recv())).await {
                Either::Left((Some(message), _)) => message,
                Either::Left((None, _)) => break, // Channel has been closed
                Either::Right(_) => break // Server has been closed
            };
            
            // Just disconnect the player if the serialization fails.
            let Ok(json_message) = message.to_json_string() else {break};

            match tcp_sender.send(Message::text(json_message)).await {
                Ok(_) => {},
                Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => break,
                Err(err) => {
                    log!(error "Connection"; "Failed to send packet. {}", err);
                    break
                },
            }
        }
        // We don't care if there is an error -- we're not using the connection anymore anyway.
        let _ = tcp_sender.close().await;
    });

    let recieve_over_tcp = {
        let listener = listener.clone();
        // It would be nice if we could avoid this ... but it's only once per connection, so it's fine.
        let connection = connection.clone();

        tokio::spawn(async move {
            while let Some(Ok(message)) = tcp_reciever.next().await {
                let Ok(mut listener) = listener.lock() else {
                    let _ = crash_signal.0.send(());
                    return;
                };
        
                listener.on_message(&connection, &message);
            }
        })
    };
    
    // When either future is complete, that means it has disconnected
    future::select(send_over_tcp, recieve_over_tcp).await;

    Ok(connection)
}
