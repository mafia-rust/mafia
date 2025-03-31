use std::{collections::HashMap, net::SocketAddr, ops::Mul, time::Duration};

use tokio_tungstenite::tungstenite::Message;

use crate::{
    lobby::{lobby_client::LobbyClientID, Lobby}, 
    packet::ToClientPacket, 
    websocket_connections::connection::Connection
};


pub trait WebsocketListener{
    fn on_connect(&mut self, connection: &Connection);
    fn on_disconnect(&mut self, connection: &Connection);
    fn on_message(&mut self, connection: &Connection, message: &Message);
}


struct ListenerClient {
    connection: Connection,
    location: ListenerClientLocation,
    last_ping: tokio::time::Instant,
}
impl ListenerClient{
    const PONG_INTERVAL: Duration = Duration::from_secs(5);

    fn new(connection: Connection) -> Self {
        Self {
            connection,
            location: ListenerClientLocation::OutsideLobby,
            last_ping: tokio::time::Instant::now(),
        }
    }
    fn on_ping(&mut self) {
        self.last_ping = tokio::time::Instant::now();
    }
    fn ping_timed_out(&self) -> bool {
        self.last_ping.elapsed() > Self::PONG_INTERVAL.mul(2)
    }
    fn tick(&mut self) {
        if Self::PONG_INTERVAL < tokio::time::Instant::now().saturating_duration_since(self.last_ping) {
            self.connection.send(ToClientPacket::Pong);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ListenerClientLocation {
    InLobby{
        room_code: RoomCode,
        lobby_client_id: LobbyClientID,
    },
    OutsideLobby
}

pub struct WebsocketListeners {
    listeners: Vec<Box<dyn WebsocketListener>>,


    lobbies: HashMap<RoomCode, Lobby>,
    clients: HashMap<SocketAddr, ListenerClient>,
}
impl WebsocketListeners{
    pub fn on_connect(&mut self, connection: &Connection) {
        for listener in &mut self.listeners{
            listener.on_connect(connection);
        }
    }
    pub fn on_disconnect(&mut self, connection: Connection) -> Result<(), &'static str> {
        for listener in &mut self.listeners{
            listener.on_disconnect(&connection);
        }
        Ok(())
    }
    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        for listener in &mut self.listeners{
            listener.on_message(connection);
        }
    }
}