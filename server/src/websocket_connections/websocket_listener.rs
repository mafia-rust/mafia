use crate::{websocket_connections::{connection::Connection, ForceLock}, listener::Listener, log};
use tokio_tungstenite::tungstenite::Message;
use std::{net::SocketAddr, sync::{Arc, Mutex}, pin::pin};

use futures_util::{future::{self, Either}, StreamExt, SinkExt};

use tokio::sync::{mpsc, broadcast};
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(server_address: &str) {
    #[expect(clippy::panic, reason = "Server cannot start without TCP listener")]
    let tcp_listener = TcpListener::bind(&server_address).await.unwrap_or_else(|err| {
        panic!("Failed to bind websocket server to address {server_address}: {err}")
    });
    
    let mut crash_signal = broadcast::channel(1);

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

    log!(important "Server"; "Started listening on {server_address}");

    loop {
        let (stream, client_address) = match future::select(
            pin!(tcp_listener.accept()), 
            pin!(crash_signal.1.recv())
        ).await {
            Either::Left((Ok((stream, client_address)), _)) => (stream, client_address),
            Either::Left((Err(_), _)) => continue, // TCP connection failed
            Either::Right(_) => break // Received crash signal
        };
        
        let event_listener = event_listener.clone();
        let crash_signal = (crash_signal.0.clone(), crash_signal.1.resubscribe());

        tokio::spawn(async move {
            if let Ok(connection) = handle_connection(stream, client_address, event_listener.clone(), crash_signal).await {
                match event_listener.force_lock().on_disconnect(connection) {
                    Ok(()) => log!(important "Connection"; "Disconnected {}", client_address),
                    Err(reason) => log!(error "Connection"; "Failed to disconnect {}: {}", client_address, reason)
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
    client_address: SocketAddr, 
    listener: Arc<Mutex<Listener>>,
    mut crash_signal: (broadcast::Sender<()>, broadcast::Receiver<()>)
) -> Result<Connection, ConnectionError> {
    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(ws_stream) => ws_stream,
        Err(error) => {
            log!(info "Connection"; "Failed to accept websocket handshake with {}: {}", client_address, error);
            return Err(ConnectionError);
        }
    };

    // Messages in this channel get received and rerouted to the client over TCP
    let (mpsc_sender, mut mpsc_receiver) = mpsc::unbounded_channel();

    let (mut tcp_sender, mut tcp_receiver) = ws_stream.split();
    
    let connection = {
        let Ok(mut listener) = listener.lock() else {
            let _ = crash_signal.0.send(());
            let _ = tcp_sender.close().await;
            return Err(ConnectionError)
        };
        let connection = Connection::new(mpsc_sender, client_address);
        log!(important "Connection"; "Connected: {}", client_address);
        listener.on_connect(&connection);
        connection
    };
    
    // Route MPSC packets to client via TCP
    let send_over_tcp = tokio::spawn(async move {
        loop {
            let message = match future::select(pin!(mpsc_receiver.recv()), pin!(crash_signal.1.recv())).await {
                Either::Left((Some(message), _)) => message,
                Either::Left((None, _)) => break, // Channel has been closed
                Either::Right(_) => break // Server has been closed
            };
            
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
        let _ = tcp_sender.close().await;
    });

    let receive_over_tcp = {
        let listener = listener.clone();
        let connection = connection.clone();

        tokio::spawn(async move {
            while let Some(Ok(message)) = tcp_receiver.next().await {
                let Ok(mut listener) = listener.lock() else {
                    let _ = crash_signal.0.send(());
                    return;
                };
        
                listener.on_message(&connection, &message);
            }
        })
    };
    
    // When either future is complete, that means it has disconnected
    future::select(send_over_tcp, receive_over_tcp).await;

    Ok(connection)
}
