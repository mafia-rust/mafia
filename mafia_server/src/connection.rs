use std::net::SocketAddr;

use futures_channel::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub struct Connection{
    tx: mpsc::UnboundedSender<Message>,
    rx: mpsc::UnboundedReceiver<Message>,
    address: SocketAddr,
    //lobby
    //player
}
impl Connection{
    pub fn new(
        tx : UnboundedSender<Message>,
        rx : UnboundedReceiver<Message>,
        address: SocketAddr
    )->Self{

        tx.unbounded_send(Message::Text("Connection Established!!".to_owned()));
        
        Self { tx, rx, address }
    }
}