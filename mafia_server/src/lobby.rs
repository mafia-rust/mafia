use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::PlayerID}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}};

pub struct Lobby {
    game: Option<Game>,
    player_names: HashMap<PlayerID, String>,
}

type LobbyID = String;

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            game: None, 
            player_names: HashMap::new(),
        }
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<Message>, player_id: PlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName { name } => {
                self.player_names.insert(player_id, name.clone());
                send.send(Message::text(ToClientPacket::YourName { name }.to_json_string()));
            },
            ToServerPacket::StartGame => {
                send.send(Message::Text(ToClientPacket::OpenGameMenu.to_json_string()));
            },
            ToServerPacket::Kick => todo!(),
            ToServerPacket::SetRoleList => todo!(),
            ToServerPacket::SetPhaseTimes => todo!(),
            ToServerPacket::SetInvestigatorResults => todo!(),
            _ => {

            }
        }
    }
}