use std::net::SocketAddr;

use tokio::sync::mpsc::UnboundedSender;

use crate::packet::ToClientPacket;

#[derive(Debug, Clone)]
pub struct Connection {
    tx: ClientSender,
    address: SocketAddr,
}

impl Connection {
    pub fn new(tx: UnboundedSender<ToClientPacket>, address: SocketAddr) -> Self {
        Self { tx: ClientSender { tx }, address }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }
    pub fn get_sender(&self) -> ClientSender {
        self.tx.clone()
    }
    pub fn send(&self, message: ToClientPacket) {
        self.tx.send(message);
    }
}

impl PartialEq for Connection{
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
impl Eq for Connection {}

#[derive(Debug, Clone)]
pub struct ClientSender {
    tx: UnboundedSender<ToClientPacket>
}

impl ClientSender {
    /// Send a message to the client.
    pub fn send(&self, message: ToClientPacket) {
        let _ = self.tx.send(message);
    }
}