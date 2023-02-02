use crate::{lobby::Lobby, network::connection::Connection};
use tokio_tungstenite::tungstenite::client;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
};

use futures_util::{future::{self}, StreamExt, TryStreamExt, SinkExt};

use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};

pub async fn create_ws_server(address: &str, clients: Arc<Mutex<HashMap<SocketAddr, Connection>>>){

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&address).await.expect("address and port should be valid. Should be 127.0.0.1:8081");  //panic if address is invalid

    println!("Listening on: {}", address);

    // Handle each incoming connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone()));
    }

    //this thread will never close i guess
}

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, clients: Arc<Mutex<HashMap<SocketAddr, Connection>>>) {
    println!("Incoming TCP connection from: {}", addr);

    // Upgrade the raw stream to a WebSocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.unwrap(); //if handshake doesnt work panic

    println!("WebSocket connection established: {}\n", addr);
    

    //2 unboundeds, one for sending through this thread to clients, other for client sending through this thread
    let (transmitter_to_client, mut reciever_to_client) = mpsc::unbounded_channel();
    let (transmitter_from_client, reciever_from_client) = mpsc::unbounded_channel();

    //messaging this client over websocket
    let (mut to_client, from_client) = ws_stream.split();

    //make unbounded things actually route to websockets
    //let send_to_client = reciever_to_client.map(Ok).forward(to_client);
    let send_to_client = tokio::spawn(async move {

        loop  {
            if let Some(m) = reciever_to_client.recv().await {
                to_client.send(m).await.unwrap();
            } else {
                break;
            };
        }

    });

    let recieve_from_client = from_client.try_for_each(|msg|{
        transmitter_from_client.send(msg).unwrap();

        future::ok(())
    });

    //create connection struct and give it ways to communicate with client
    clients.lock().unwrap().insert(
        addr.clone(),
        Connection::new(transmitter_to_client, reciever_from_client, addr)
    );

    futures_util::pin_mut!(send_to_client, recieve_from_client);    //no clue what this does but example code told me to do it
    future::select(send_to_client, recieve_from_client).await;  //when both are complete then that means its disconnected

    clients.lock().unwrap().remove(&addr);
    println!("{:?}", clients.lock().unwrap());

    println!("{} disconnected", &addr);
}
