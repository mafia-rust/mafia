use std::{net::SocketAddr, ops::Mul, time::Duration};

use crate::{lobby::{lobby_client::LobbyClientID, Lobby}, packet::ToClientPacket, websocket_connections::connection::{ClientSender, Connection}};

use super::{RoomCode, WebsocketListener};


///  to be valid as long as it is never stored and not used after an obvious disconnect
/// Tehe
pub(super) struct ClientReference{
    addr: SocketAddr,
}
impl ClientReference{
    pub(super) fn new(addr: &SocketAddr, listener: &WebsocketListener)->Option<Self>{
        let _ = listener.get_client(addr)?;
        unsafe{Some(Self::new_unchecked(*addr))}
    }
    pub(super) unsafe fn new_unchecked(addr: SocketAddr)->Self{
        Self { addr }
    }
    pub(super) fn all_clients(listener: &WebsocketListener) -> impl Iterator<Item=ClientReference> {
        unsafe{
            listener
                .clients()
                .keys()
                .copied()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|addr|Self::new_unchecked(addr))
        }
    }

    pub(super) fn deref<'a>(&self, listener: &'a WebsocketListener)->&'a Client{
        listener.get_client(&self.addr).expect("ClientReference")
    }
    pub(super) fn deref_mut<'a>(&self, listener: &'a mut WebsocketListener)->&'a mut Client{
        listener.get_client_mut(&self.addr).expect("ClientReference")
    }

    pub(super) fn send(&self, listener: &WebsocketListener, message: ToClientPacket) {
        self.deref(listener).send(message);
    }
    pub(super) fn address<'a>(&self, listener: &'a WebsocketListener)->&'a SocketAddr{
        self.deref(listener).connection.address()
    }
    pub(super) fn sender(&self, listener: &WebsocketListener)->ClientSender{
        self.deref(listener).connection.sender()
    }
    pub(super) fn location<'a>(&self, listener: &'a WebsocketListener)->&'a ClientLocation{
        &self.deref(listener).location
    }
    pub(super) fn set_location(&self, listener: &mut WebsocketListener, loc: ClientLocation){
        self.deref_mut(listener).location = loc
    }

    pub(super) fn get_lobby<'a>(&self, listener: &'a WebsocketListener)->Result<(&'a Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        self.location(listener).clone().get_lobby(listener)
    }
    pub(super) fn get_lobby_mut<'a>(&self, listener: &'a mut WebsocketListener)->Result<(&'a mut Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        self.location(listener).clone().get_lobby_mut(listener)
    }
    pub(super) fn in_room(&self, listener: &WebsocketListener, room_code: RoomCode)->bool{
        self.deref(listener).in_room(room_code)
    }

    pub(super) fn tick(&self, listener: &mut WebsocketListener){
        self.deref_mut(listener).tick();
    }
    pub(super) fn ping_timed_out(&self, listener: &WebsocketListener)->bool{
        self.deref(listener).ping_timed_out()
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
        self.connection.send(packet);
    }
    pub(super) fn location(&self)->&ClientLocation{
        &self.location
    }
    pub(super) fn in_room(&self, room_code: RoomCode)->bool{
        self.location().in_room(room_code)
    }

    // ligma, im commenting
    // pub(super) fn get_lobby<'a>(&self, listener: &'a WebsocketListener)->Result<(&'a Lobby, RoomCode, LobbyClientID),GetLobbyError>{
    //     self.location.get_lobby(listener)
    // }

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum ClientLocation {
    InLobby{
        room_code: RoomCode,
        lobby_client_id: LobbyClientID,
    },
    OutsideLobby
}
impl ClientLocation{
    pub(super) fn in_room(&self, room_code: RoomCode)->bool{
        let Self::InLobby { room_code: b, .. } = &self else {return false};
        room_code == *b 
    }
    pub(super) fn get_lobby<'a>(&self, listener: &'a WebsocketListener)->Result<(&'a Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        let ClientLocation::InLobby{room_code, lobby_client_id} = &self else {return Err(GetLobbyError::NotInLobby)};
        let Some(lobby) = listener.get_lobby(room_code) else {return Err(GetLobbyError::LobbyDoesntExist)};
        Ok((lobby, *room_code, *lobby_client_id))
    }
    pub(super) fn get_lobby_mut<'a>(&self, listener: &'a mut WebsocketListener)->Result<(&'a mut Lobby, RoomCode, LobbyClientID),GetLobbyError>{
        let ClientLocation::InLobby{room_code, lobby_client_id} = &self else {return Err(GetLobbyError::NotInLobby)};
        let Some(lobby) = listener.get_lobby_mut(room_code) else {return Err(GetLobbyError::LobbyDoesntExist)};
        Ok((lobby, *room_code, *lobby_client_id))
    }
}

pub(super) enum GetLobbyError{
    NotInLobby,
    LobbyDoesntExist,
}