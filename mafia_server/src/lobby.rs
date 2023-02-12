use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::PlayerID}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}};

pub struct Lobby {
    game: Option<Game>,
    player_names: Vec<(UnboundedSender<Message>, String)>,  //index = id - 1;
}

pub type LobbyID = usize;

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            game: None, 
            player_names: Vec::new(),
        }
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<Message>, player_id: PlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName { name } => {
                self.player_names.insert(player_id, (send.clone(), name.clone()));
                send.send(Message::text(ToClientPacket::YourName { name }.to_json_string()));
            },
            ToServerPacket::StartGame => {
                for player in self.player_names.iter(){
                    player.0.send(Message::Text(ToClientPacket::OpenGameMenu.to_json_string()));
                    //send.send(Message::Text(ToClientPacket::OpenGameMenu.to_json_string()));
                }
                
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