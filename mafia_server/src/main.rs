
use mafia_server::lobby::{Lobby, self};
use mafia_server::connection::Connection;

use tokio_tungstenite::WebSocketStream;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    collections::{HashSet, HashMap},
};

use futures_util::{StreamExt, TryStreamExt, future};

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main()->Result<(), ()>{
    println!("Hello, world!");

    let mut lobbies = vec![];
    let mut clients: vec![];

    let server_future = create_ws_server();

    lobbies.push(Lobby::new());
    clients.push()

    
    server_future.await;
    Ok(())
}



async fn create_ws_server(){

    // Server address
    let addr = "127.0.0.1:8081";

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await.expect("address and port should be valid. Should be 127.0.0.1:8081");
    println!("Listening on: {}", addr);

    // Handle each incoming connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    // Upgrade the raw stream to a WebSocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await
        .expect("Error during the WebSocket handshake occurred");

    println!("WebSocket connection established: {}", addr);

    //messaging this client over websocket
    let (outgoing, incoming) = ws_stream.split();

    // print incoming messages
    let print_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}:\n{}", addr, msg.to_text().unwrap());
        future::ok(())
    });


    futures_util::pin_mut!(print_incoming);
    print_incoming.await;

    println!("{} disconnected", &addr);
}
