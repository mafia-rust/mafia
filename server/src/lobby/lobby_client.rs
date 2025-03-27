use std::collections::VecDeque;
use std::time::Instant;

use serde::Serialize;

use crate::game::player::PlayerReference;
use crate::game::spectator::spectator_pointer::SpectatorPointer;
use crate::game::Game;
use crate::{client_connection::ClientConnection, packet::ToClientPacket, websocket_connections::connection::ClientSender};

use super::game_client::GameClient;
use super::GameClientLocation;

pub type LobbyClientID = u32;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyClient{
    pub connection: ClientConnection,
    pub ready: Ready,
    pub client_type: LobbyClientType,
    
    #[serde(skip)]
    pub last_message_times: VecDeque<Instant>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Ready {
    Host,
    Ready,
    NotReady,
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
            connection: ClientConnection::Connected(connection),
            ready: if host { Ready::Host } else { Ready::NotReady },
            client_type: LobbyClientType::Player{name},
            last_message_times: VecDeque::new()
        }
    }
    pub fn new_from_game_client(game: &Game, game_client: GameClient)->Self{

        match game_client.client_location {
            GameClientLocation::Player(index) => {
                let player_ref = unsafe { PlayerReference::new_unchecked(index) };
                LobbyClient{
                    connection: player_ref.connection(game).clone(),
                    ready: if game_client.host { Ready::Host } else { Ready::NotReady },
                    client_type: LobbyClientType::Player{name: player_ref.name(game).to_string()},
                    last_message_times: VecDeque::new()
                }
            },
            GameClientLocation::Spectator(index) => {
                let spectator_pointer = SpectatorPointer::new(index);
                LobbyClient{
                    connection:spectator_pointer.connection(game),
                    ready: if game_client.host { Ready::Host } else { Ready::Ready },
                    client_type: LobbyClientType::Spectator,
                    last_message_times: VecDeque::new()
                }
            }
        }

        
    }
    pub fn set_host(&mut self) {
        self.ready = Ready::Host;
    }
    pub fn relinquish_host(&mut self) {
        self.ready = Ready::NotReady;
    }

    pub fn is_host(&self) -> bool {
        self.ready == Ready::Host
    }

    pub fn is_spectator(&self) -> bool {
        matches!(self.client_type, LobbyClientType::Spectator)
    }

    pub fn send(&self, message: ToClientPacket) {
        if let ClientConnection::Connected(ref sender) = self.connection {
            sender.send(message);
        }
    }
}