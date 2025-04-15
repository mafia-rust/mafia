
pub mod game_client;
pub mod on_client_message;
pub mod name_validation;

use std::time::Duration;

use super::lobby::Lobby;

use crate::{
    game::Game, packet::{RoomPreviewData, RejectJoinReason, ToClientPacket}, websocket_connections::connection::ClientSender
};


pub type RoomClientID = u32;

#[enum_delegate::register]
pub trait RoomState {
    fn send_to_client_by_id(&self, room_client_id: RoomClientID, packet: ToClientPacket);
    fn join_client(&mut self, send: &ClientSender) -> Result<JoinClientData, RejectJoinReason>;
    fn initialize_client(&mut self, room_client_id: RoomClientID, send: &ClientSender);
    fn remove_client(&mut self, id: RoomClientID) -> RemoveClientData;
    fn remove_client_rejoinable(&mut self, id: RoomClientID) -> RemoveClientData;
    fn rejoin_client(&mut self, send: &ClientSender, room_client_id: RoomClientID) -> Result<JoinClientData, RejectJoinReason>;
    fn tick(&mut self, time_passed: Duration) -> TickData;
    fn get_preview_data(&self) -> RoomPreviewData;
    fn is_host(&self, room_client_id: RoomClientID)->bool;
}

#[enum_delegate::implement(RoomState)]
pub enum Room {
    Lobby(Lobby),
    Game(Game),
}

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

impl Room {
    pub fn new() -> Room {
        Room::Lobby(Lobby::new())
    }
}
