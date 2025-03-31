use std::{net::SocketAddr, ops::Mul, time::Duration};

use crate::{lobby::{lobby_client::LobbyClientID, Lobby}, packet::ToClientPacket, websocket_connections::connection::Connection};

use super::{RoomCode, WebsocketListener};


/// Garunteed to be valid as long as its stored
pub(super) struct ClientReference{
    addr: SocketAddr
}
impl ClientReference{
    pub(super) fn new(addr: &SocketAddr, listener: &WebsocketListener)->Option<Self>{
        if !listener.clients.contains_key(addr) {return None}
        unsafe{Some(Self::new_unchecked(*addr))}
    }
    pub(super) unsafe fn new_unchecked(addr: SocketAddr)->Self{
        Self { addr }
    }
}


pub(super) struct Client {
    connection: Connection,
    location: ClientLocation,
    last_ping: tokio::time::Instant,
}
impl Client{
    const PONG_INTERVAL: Duration = Duration::from_secs(5);

    pub(super) fn new(connection: Connection) -> Self {
        Self {
            connection,
            location: ClientLocation::OutsideLobby,
            last_ping: tokio::time::Instant::now(),
        }
    }
    pub(super) fn on_ping(&mut self) {
        self.last_ping = tokio::time::Instant::now();
    }
    pub(super) fn ping_timed_out(&self) -> bool {
        self.last_ping.elapsed() > Self::PONG_INTERVAL.mul(2)
    }
    pub(super) fn tick(&mut self) {
        if Self::PONG_INTERVAL < tokio::time::Instant::now().saturating_duration_since(self.last_ping) {
            self.connection.send(ToClientPacket::Pong);
        }
    }
    pub(super) fn send(&self, packet: ToClientPacket){
        &self.connection.send(packet);
    }

    pub(super) fn location(&self)->&ClientLocation{
        &self.location
    }
    pub(super) fn get_lobby<'a>(&self, listener: &'a WebsocketListener)->Result<(&'a Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        self.location.get_lobby(listener)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ClientLocation {
    InLobby{
        room_code: RoomCode,
        lobby_client_id: LobbyClientID,
    },
    OutsideLobby
}
impl ClientLocation{
    pub(super) fn get_lobby<'a>(&self, listener: &'a WebsocketListener)->Result<(&'a Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        let ClientLocation::InLobby{room_code, lobby_client_id} = &self else {return Err(GetLobbyError::NotInLobby)};
        let lobby = listener.lobbies.get(room_code) else {return Err(GetLobbyError::LobbyDoesntExist)};
        Ok((lobby, *room_code, *lobby_client_id))
    }
}

pub(super) enum GetLobbyError{
    NotInLobby,
    LobbyDoesntExist,
}