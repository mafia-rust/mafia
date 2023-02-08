use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use super::packet::ToClientPacket;

#[derive(Debug)]
pub struct Connection {
    tx: UnboundedSender<Message>,
    address: SocketAddr,
}

impl Connection {
    pub fn new(tx: UnboundedSender<Message>, address: SocketAddr) -> Self {
        Self { tx, address }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn send(&self, message: ToClientPacket) {
        self.tx.send(Message::text(message.to_json_string()));
    }
}

pub trait ConnectionEventListener {
    fn on_connect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_disconnect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_message(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message);
}