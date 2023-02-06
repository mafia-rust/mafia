use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub struct Connection{
    tx: UnboundedSender<Message>,
    address: SocketAddr,
}
impl core::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection").field("tx", &self.tx).field("address", &self.address).finish()
    }
}
impl Connection{
    pub fn new(
        tx : UnboundedSender<Message>,
        address: SocketAddr,
    )->Self{

        tx.send(Message::Text("Connection Established!!".to_owned()));

        Self { tx, address }
    }
    pub fn get_adress(&self)->&SocketAddr{
        &self.address
    }
    pub fn send(&self, message: Message){
        self.tx.send(message);
    }
}

pub trait ConnectionEventListener {
    fn on_connect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_disconnect(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection);
    fn on_message(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message);
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}