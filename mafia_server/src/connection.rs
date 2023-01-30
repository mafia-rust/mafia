use std::net::SocketAddr;

//use futures_channel::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub struct Connection{
    tx: UnboundedSender<Message>,
    rx: UnboundedReceiver<Message>,
    address: SocketAddr,
    //lobby
    //player
}
impl Connection{
    pub fn new(
        tx : UnboundedSender<Message>,
        mut rx : UnboundedReceiver<Message>,
        address: SocketAddr
    )->Self{

        tx.send(Message::Text("Connection Established!!".to_owned()));
    
        Self { tx, rx, address }
    }
}

