use crate::{lobby::Lobby, websocket_connections::connection::{Connection, ConnectionEventListener, self}, log, listener::Listener};
use tokio_native_tls::{TlsAcceptor, native_tls::{self, Identity}};
use tokio_tungstenite::tungstenite::{client, Message};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
};

use futures_util::{future, StreamExt, TryStreamExt, SinkExt};

use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(address: &str) {
    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&address).await.unwrap_or_else(|_| panic!("Invalid address: {address}"));
    
    // let identity = Identity::from_pkcs12(include_bytes!("../cert.pfx"), "password").unwrap();
    // let acceptor = native_tls::TlsAcceptor::new(identity)

    // TODO: Consider combining these into a `GlobalServerData` struct
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let event_listener = Arc::new(Mutex::new(Listener::new()));

    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
    println!("{}", log::notice("Mafia Server started!\n"));
    println!("Listening on: {}\n", log::important(address));
    println!("Log output:");

    // Handle each incoming connection in a separate task. This runs forever
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), event_listener.clone()));
    }
}

pub async fn handle_connection(
    raw_stream: TcpStream, 
    addr: SocketAddr, 
    clients_mutex: Arc<Mutex<HashMap<SocketAddr, Connection>>>, 
    mut listener: Arc<Mutex<impl ConnectionEventListener>>
) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await
        .expect("Failed to accept websocket handshake");

    // Sending to client MPSC (This gets recieved by this thread and rerouted to the client over TCP)
    let (transmitter_to_client, mut reciever_to_client) = mpsc::unbounded_channel();

    // Client TCP connection
    let (mut to_client, from_client) = ws_stream.split();

    // Add connection to the hashmap to keep track
    {
        let mut clients = clients_mutex.lock().unwrap();
        clients.insert(addr, Connection::new(transmitter_to_client, addr)); 
    }
    
    // Route MPSC packets to client via TCP
    let send_to_client = tokio::spawn(async move {
        while let Some(m) = reciever_to_client.recv().await {
            match to_client.send(Message::text(m.to_json_string())).await{
                Ok(_) => {},
                Err(_) => {break;},
            }
        }
        to_client.close();
    });

    let recieve_from_client = from_client.try_for_each(|message|{
        let clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();

        listener.lock().unwrap().on_message(&clients, connection, &message);
            
        future::ok(())
    });

    {
        let mut clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();

        listener.lock().unwrap().on_connect(&clients, connection);
    }
    
    futures_util::pin_mut!(send_to_client, recieve_from_client);//pinmut needed for select
    future::select(send_to_client, recieve_from_client).await;

    // When either future is complete, that means it has disconnected
    // Handle disconnection
    {
        let mut clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();
        
        listener.lock().unwrap().on_disconnect(&clients, connection);
        clients.remove(&addr);
    }
}
