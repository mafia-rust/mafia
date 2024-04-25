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
    pub fn new_from_game_client(game: &Game, game_client: GameClient)->Self{

        match game_client.client_location {
            GameClientLocation::Player(index) => {
                let player_ref = PlayerReference::new_unchecked(index);
                LobbyClient{
                    connection: player_ref.connection(game).clone(),
                    host: game_client.host,
                    client_type: LobbyClientType::Player{name: player_ref.name(game).to_string()}
                }
            },
            GameClientLocation::Spectator(index) => {
                let spectator_pointer = SpectatorPointer::new(index);
                LobbyClient{
                    connection:spectator_pointer.connection(game),
                    host: game_client.host,
                    client_type: LobbyClientType::Spectator
                }
            }
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