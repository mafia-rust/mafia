
pub mod lobby_client;
pub mod game_client;
pub mod on_client_message;
pub mod lobby_state;
pub mod on_lobby_message;
pub mod name_validation;

use std::time::Duration;

use game_client::GameClientLocation;
use lobby_state::Lobby;

use crate::{
    game::Game, packet::{LobbyPreviewData, RejectJoinReason, ToClientPacket}, websocket_connections::connection::ClientSender
};


use self::lobby_client::RoomClientID;


pub enum RoomState {
    Lobby(Lobby),
    Game(Game),
}
pub const GAME_DISCONNECT_TIMER_SECS: u16 = 60 * 2;

#[must_use = "Send the accept join packet"]
pub struct JoinClientData {
    pub id: RoomClientID,
    pub in_game: bool,
    pub spectator: bool 
}

#[must_use = "You may need to close the room"]
pub struct RemoveClientData {
    pub close_room: bool,
}

#[must_use = "You may need to close the room"]
pub struct TickData {
    pub close_room: bool,
}

impl RoomState {
    pub fn new() -> RoomState {
        RoomState::Lobby(Lobby::new())
    }

    pub fn send_to_client_by_id(&self, lobby_client_id: RoomClientID, packet: ToClientPacket) {
        match self {
            RoomState::Lobby(lobby) => lobby.send_to_client_by_id(lobby_client_id, packet),
            RoomState::Game(game) => game.send_to_client_by_id(lobby_client_id, packet),
        }
    }

    pub fn join_client(&mut self, send: &ClientSender) -> Result<JoinClientData, RejectJoinReason>{
        match self {
            RoomState::Lobby(lobby) => lobby.join_client(send),
            RoomState::Game(game) => game.join_client(send),
        }
    }

    pub fn initialize_client(&mut self, lobby_client_id: RoomClientID, send: &ClientSender) {
        match self {
            RoomState::Lobby(lobby) => lobby.initialize_client(lobby_client_id, send),
            RoomState::Game(game) => game.initialize_client(lobby_client_id, send),
        }
    }

    pub fn remove_client(&mut self, id: RoomClientID) -> RemoveClientData {
        match self {
            RoomState::Lobby(lobby) => lobby.remove_client(id),
            RoomState::Game(game) => game.remove_client(id),
        }
    }

    pub fn remove_client_rejoinable(&mut self, id: RoomClientID) -> RemoveClientData {
        match self {
            RoomState::Lobby(lobby) => lobby.remove_client_rejoinable(id),
            RoomState::Game(game) => game.remove_client_rejoinable(id),
        }
    }

    pub fn rejoin_client(&mut self, send: &ClientSender, lobby_client_id: RoomClientID) -> Result<JoinClientData, RejectJoinReason> {
        match self {
            RoomState::Lobby(lobby) => lobby.rejoin_client(send, lobby_client_id),
            RoomState::Game(game) => game.rejoin_client(send, lobby_client_id),
        }
    }

    pub fn tick(&mut self, time_passed: Duration) -> TickData {
        match self {
            RoomState::Game(game) => game.tick(time_passed),
            RoomState::Lobby(lobby) => lobby.tick(time_passed),
        }
    }

    pub fn get_preview_data(&self) -> LobbyPreviewData {
        match self {
            RoomState::Lobby(lobby) => lobby.get_preview_data(),
            RoomState::Game(game) => game.get_preview_data(),
        }
    }

    pub fn is_host(&self, lobby_client_id: RoomClientID)->bool{
        match self {
            RoomState::Lobby(lobby) => lobby.is_host(lobby_client_id),
            RoomState::Game(game) => game.is_host(lobby_client_id),
        }
    }
}
