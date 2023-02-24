use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::{PlayerIndex, Player}, settings::Settings, role_list}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}};

pub struct Lobby {
    game: Option<Game>,
    player_names: Vec<(UnboundedSender<ToClientPacket>, String)>,
}

pub type LobbyIndex = usize;

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            game: None, 
            player_names: Vec::new(),
        }
    }
    pub fn add_new_player(&mut self, player: (UnboundedSender<ToClientPacket>, String))->PlayerIndex{
        self.player_names.push(player);
        self.player_names.len() - 1
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<ToClientPacket>, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                self.player_names.insert(player_index, (send.clone(), name.clone()));
                send.send(ToClientPacket::YourName{name});
            },
            ToServerPacket::StartGame => {
                for player in self.player_names.iter(){

                    let settings = Settings::new(self.player_names.len());
                    if(self.game.is_none()){
                        player.0.send(ToClientPacket::OpenGameMenu);
                        self.game = Some(Game::new(settings, self.player_names.clone()))
                    }

                }
            },
            ToServerPacket::Kick(player_index) => {
                
            },
            ToServerPacket::SetRoleList(role_list) => todo!(),
            ToServerPacket::SetPhaseTimes(phase_times) => todo!(),
            ToServerPacket::SetInvestigatorResults(invest_results) => todo!(),
            _ => {
                if self.game.is_some(){ //TODO jack please jack help please jack plz
                    self.game.as_mut().unwrap().on_client_message(player_index, incoming_packet);
                }
            }
        }
    }
}