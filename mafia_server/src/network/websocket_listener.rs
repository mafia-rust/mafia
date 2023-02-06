use crate::{lobby::Lobby, network::connection::{Connection, ConnectionEventListener, self}};
use tokio_tungstenite::tungstenite::{client, Message};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
};

use futures_util::{future::{self}, StreamExt, TryStreamExt, SinkExt};

use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(
    address: &str, 
    clients: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
    mut event_listener: Box<impl ConnectionEventListener + Send + 'static>,
) {
    let event_listener = Arc::new(Mutex::new(*event_listener));

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&address).await.expect("address and port should be valid. Should be 127.0.0.1:8081");  //panic if address is invalid

    println!("Listening on: {}\n", address);

    // Handle each incoming connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), event_listener.clone()));
    }

    //this thread will never close i guess
}

pub async fn handle_connection(
    raw_stream: TcpStream, 
    addr: SocketAddr, 
    clients_mutex: Arc<Mutex<HashMap<SocketAddr, Connection>>>, 
    mut listener: Arc<Mutex<impl ConnectionEventListener>>
) {
    //println!("Incoming TCP connection from: {}", addr);

    // Upgrade the raw stream to a WebSocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.unwrap(); //if handshake doesnt work panic
    //println!("WebSocket connection established: {}\n", addr);
    
    //sending to client mpsc
    let (transmitter_to_client, mut reciever_to_client) = mpsc::unbounded_channel();

    //sending to clinet tcp
    let (mut to_client, from_client) = ws_stream.split();


    //create connection struct and give it ways to communicate with client
    {
        let mut clients = clients_mutex.lock().unwrap();
        clients.insert(
            addr.clone(),
            Connection::new(transmitter_to_client, addr)
        ); 
    }
    
    // route between unbounded senders and websockets
    let send_to_client = tokio::spawn(async move {
        while let Some(m) = reciever_to_client.recv().await {
            to_client.send(m).await.unwrap();
        }
        to_client.close();
    });

    let recieve_from_client = from_client.try_for_each(|message|{
        let clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();

        // @Jack-Papel: @ItsSammyM why clone here?
        listener.clone().lock().unwrap().on_message(&clients, connection, &message);
            
        future::ok(())
    });

    {
        let mut clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();

        listener.lock().unwrap().on_connect(&clients, connection);
    }
    
    // @ItsSammyM: No clue what this does but example code told me to do it
    futures_util::pin_mut!(send_to_client, recieve_from_client);
    future::select(send_to_client, recieve_from_client).await;

    // When both are complete then that means it's disconnected
    {
        let mut clients = clients_mutex.lock().unwrap();
        let connection = clients.get(&addr).unwrap();
        
        listener.lock().unwrap().on_disconnect(&clients, connection);
        clients.remove(&addr);
    }
    

    // println!("{} disconnected", &addr);
}
