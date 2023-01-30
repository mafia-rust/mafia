
use mafia_server::lobby::Lobby;
use mafia_server::connection::Connection;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_util::{future::{self}, StreamExt, TryStreamExt};
use futures_channel::mpsc;

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main()->Result<(), ()>{
    println!("Hello, world!");

    let mut lobbies = vec![];
    let clients: Arc<Mutex<Vec<Connection>>> = Arc::new(Mutex::new(vec![]));

    let server_future = create_ws_server("127.0.0.1:8081", clients);

    lobbies.push(Lobby::new());

    
    server_future.await;
    Ok(())
}



async fn create_ws_server(address: &str, clients: Arc<Mutex<Vec<Connection>>>){

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&address).await.expect("address and port should be valid. Should be 127.0.0.1:8081");  //panic if address is invalid

    println!("Listening on: {}", address);

    // Handle each incoming connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone()));
    }

    //this thread will never close i guess
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, clients: Arc<Mutex<Vec<Connection>>>) {
    println!("Incoming TCP connection from: {}", addr);

    // Upgrade the raw stream to a WebSocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.unwrap(); //if handshake doesnt work panic

    println!("WebSocket connection established: {}\n", addr);
    

    //2 unboundeds, one for sending through this thread to clients, other for client sending through this thread
    let (transmitter_to_client, reciever_to_client) = mpsc::unbounded();
    let (transmitter_from_client, reciever_from_client) = mpsc::unbounded();

    //create connection struct and give it ways to communicate with client
    clients.lock().unwrap().push(
        Connection::new(transmitter_to_client, reciever_from_client, addr)
    );

    //messaging this client over websocket
    let (to_client, from_client) = ws_stream.split();

    //make unbounded things actually route to websockets
    let send_to_client = reciever_to_client.map(Ok).forward(to_client);
    let recieve_from_client = from_client.try_for_each(|msg|{
        transmitter_from_client.unbounded_send(msg).unwrap();

        future::ok(())
    });

    
    futures_util::pin_mut!(send_to_client, recieve_from_client);    //no clue what this does but example code told me to do it
    future::select(send_to_client, recieve_from_client).await;  //when both are complete then that means its disconnected

    println!("{} disconnected", &addr);
}
