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
){
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
    clients: Arc<Mutex<HashMap<SocketAddr, Connection>>>, 
    mut listener: Arc<Mutex<impl ConnectionEventListener>>
){
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
        let mut clients_unlocked1 = clients.lock().unwrap();
        clients_unlocked1.insert(
            addr.clone(),
            Connection::new(transmitter_to_client, addr)
        ); 
    }
    
    //route between unbounded senders and websockets
    let send_to_client = tokio::spawn(async move {
        loop  {
            if let Some(m) = reciever_to_client.recv().await {
                to_client.send(m).await.unwrap();
            } else {
                to_client.close();
                break;
            };
        }
    });
    let recieve_from_client = from_client.try_for_each(|message|{

        let clients_unlocked2 = clients.lock().unwrap();
        let connection = clients_unlocked2.get(&addr).unwrap();

        listener.clone().lock().unwrap().on_message(&clients_unlocked2, connection, &message);
            
        future::ok(())
    });

    {
        let mut clients_unlocked3 = clients.lock().unwrap();
        let connection = clients_unlocked3.get(&addr).unwrap();

        listener.lock().unwrap().on_connect(&clients_unlocked3, connection);
    }
    

    
    futures_util::pin_mut!(send_to_client, recieve_from_client);    //no clue what this does but example code told me to do it
    future::select(send_to_client, recieve_from_client).await;

    //when both are complete then that means its disconnected
    {
        let mut clients_unlocked4 = clients.lock().unwrap();
        let connection = clients_unlocked4.get(&addr).unwrap();
        
        listener.lock().unwrap().on_disconnect(&clients_unlocked4, connection);
        clients_unlocked4.remove(&addr);
    }
    

    //println!("{} disconnected", &addr);
}
