use std::{collections::HashMap, net::SocketAddr};

use crate::{game::{Game, player::PlayerID}, network::{connection::Connection, packet::ToServerPacket}};

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
    pub fn on_client_message(player: PlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName { name } => {

            },
            ToServerPacket::StartGame => todo!(),
            ToServerPacket::Kick => todo!(),
            ToServerPacket::SetRoleList => todo!(),
            ToServerPacket::SetPhaseTimes => todo!(),
            ToServerPacket::SetInvestigatorResults => todo!(),
            _ => {

            }
        }
    }
}