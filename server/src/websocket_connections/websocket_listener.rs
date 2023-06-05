use crate::{websocket_connections::{connection::Connection, ForceLock}, log, listener::Listener};
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    collections::HashMap,
};

use futures_util::{future, StreamExt, TryStreamExt, SinkExt};

use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(address: &str) {
    // Create the event loop and TCP listener we'll accept connections on.
    let tcp_listener = TcpListener::bind(&address).await.unwrap_or_else(|err| {
        panic!("Failed to bind websocket server to address {address}: {err}")
    });

    // TODO: Consider combining these into a `GlobalServerData` struct
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let event_listener = Arc::new(Mutex::new(Listener::new()));

    Listener::start(event_listener.clone());

    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
    println!("{}", log::notice("Mafia Server started!\n"));
    println!("Listening on: {}\n", log::important(address));
    println!("Log output:");

    // Handle each incoming connection in a separate task. This runs forever
    while let Ok((stream, addr)) = tcp_listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), event_listener.clone()));
    }
}

pub async fn handle_connection(
    raw_stream: TcpStream, 
    addr: SocketAddr, 
    clients_mutex: Arc<Mutex<HashMap<SocketAddr, Connection>>>, 
    listener: Arc<Mutex<Listener>>
) {
    let Ok(ws_stream) = tokio_tungstenite::accept_async(raw_stream).await else {
        println!("{} failed to accept websocket handshake with {}", log::error("Error:"), addr);
        return
    };

    // Sending to client MPSC (This gets recieved and rerouted to the client over TCP)
    let (transmitter_to_client, mut reciever_to_client) = mpsc::unbounded_channel();

    // Client TCP connection
    let (mut to_client, from_client) = ws_stream.split();

    // Add connection to the hashmap to keep track
    {
        let mut clients = clients_mutex.force_lock();
        clients.insert(addr, Connection::new(transmitter_to_client, addr)); 
    }
    
    // Route MPSC packets to client via TCP
    let send_to_client = tokio::spawn(async move {
        while let Some(m) = reciever_to_client.recv().await {
            // Just disconnect the player if the serialization fails.
            let Ok(json_message) = m.to_json_string() else {break};

            match to_client.send(Message::text(json_message)).await {
                Ok(_) => {},
                Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => break,
                Err(err) => {
                    println!("{} while sending packet. {}", log::error("Error"), err);
                    break
                },
            }
        }
        // We don't care if there is an error -- we're not using the connection anymore anyway.
        let _ = to_client.close().await;
    });

    let recieve_from_client = from_client.try_for_each(|message| {
        let clients = clients_mutex.force_lock();
        let connection = &clients[&addr];

        // This thread should close (panic) if the listener has been poisoned.
        listener.lock().unwrap().on_message(connection, &message);

        future::ok(())
    });

    {
        let clients = clients_mutex.force_lock();
        let connection = &clients[&addr];

        match listener.lock() {
            Ok(mut listener) => listener.on_connect(connection),
            Err(_poison) => return // Don't connect a player if the listener is poisoned
        }
    }
    
    futures_util::pin_mut!(send_to_client, recieve_from_client);//pinmut needed for select
    future::select(send_to_client, recieve_from_client).await;

    // When either future is complete, that means it has disconnected
    // Handle disconnection
    {
        let mut clients = clients_mutex.force_lock();
        let connection = &clients[&addr];
        
        // Disconnect player, no matter what
        listener.force_lock().on_disconnect(connection);
        clients.remove(&addr);
    }
}
