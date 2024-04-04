use serde::Serialize;

use crate::{client_connection::ClientConnection, packet::ToClientPacket, websocket_connections::connection::ClientSender};

pub type LobbyClientID = u32;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyClient{
    pub connection: ClientConnection,
    pub host: bool,
    pub client_type: LobbyClientType,
}
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum LobbyClientType{
    Spectator,
    Player{
        name: String,
    }
}

impl LobbyClient {
    pub fn new(name: String, connection: ClientSender, host: bool)->Self{
        LobbyClient{
           connection: ClientConnection::Connected(connection), host, client_type: LobbyClientType::Player{name}
        }
    }
    pub fn set_host(&mut self) {
        self.host = true;
    }

    pub fn send(&self, message: ToClientPacket) {
        if let ClientConnection::Connected(ref sender) = self.connection {
            sender.send(message);
        }
    }
}