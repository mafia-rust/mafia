use crate::game::Game;

pub struct Lobby {

    //the idea behind the lobby thing is that it  allows players to connect before the game started
    //therefore, the game gets created and then starts at the same time
    //we couold have it so a game can be created without being started. You pick if thats a better idea, if it is you can delete this file.
    game: Option<Game>,
}

impl Lobby{
    pub fn new()->Lobby{




        create_ws_server();

        Lobby { 
            game: None 
        }
    }
}

use std::{
    collections::HashMap,
    io,
    io::Read,
    fs::File,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_util::{StreamExt, TryStreamExt, future, pin_mut};
use tokio_tungstenite::tungstenite::{self, stream};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

use tokio_native_tls::native_tls::{Identity};
use tokio_native_tls::{TlsAcceptor, TlsStream};

#[tokio::main]
async fn create_ws_server() -> core::result::Result<(), io::Error> {
    
    //https://docs.rs/native-tls/latest/native_tls/
    let mut file = File::open("identity.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();

    let identity = Identity::from_pkcs12(&identity, "hunter2").unwrap();
    let acceptor = TlsAcceptor::from(tokio_native_tls::native_tls::TlsAcceptor::new(identity).unwrap());
    let acceptor = Arc::new(acceptor);


    // Server address
    let addr = "127.0.0.1:8081";
    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    // Handle each incoming connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        let acceptor = acceptor.clone();
        let stream = acceptor.accept(stream).await.unwrap();
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}

async fn handle_connection(raw_stream: TlsStream<TcpStream>, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    // Upgrade the raw stream to a WebSocket stream
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the WebSocket handshake occurred");

    println!("WebSocket connection established: {}", addr);

    //messaging this client over websocket
    let (outgoing, incoming) = ws_stream.split();

    // Broadcast incoming messages to all other clients
    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}:\n{}", addr, msg.to_text().unwrap());
        future::ok(())
    });

    //pin_mut!(broadcast_incoming);
    broadcast_incoming.await;

    println!("{} disconnected", &addr);
}