use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use crate::packet::ToClientPacket;

#[derive(Debug)]
pub struct Connection {
    tx: UnboundedSender<ToClientPacket>,
    address: SocketAddr,
}

impl Eq for Connection{

}
impl PartialEq for Connection{
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Connection {
    pub fn new(tx: UnboundedSender<ToClientPacket>, address: SocketAddr) -> Self {
        Self { tx, address }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }
    pub fn get_sender(&self) -> UnboundedSender<ToClientPacket> {
        self.tx.clone()
    }
    pub fn send(&self, message: ToClientPacket) {
        self.tx.send(message);
    }
}

pub trait ConnectionEventListener {
    fn on_connect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_disconnect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_message(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message);
}