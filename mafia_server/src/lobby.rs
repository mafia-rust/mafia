use std::{collections::HashMap, net::SocketAddr};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::{PlayerIndex, Player}, settings::{Settings, InvestigatorResults}, role_list}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}};

pub struct Lobby {
    game: Option<Game>,
    settings: Settings,
    player_names: Vec<(UnboundedSender<ToClientPacket>, String)>,
}

pub type LobbyIndex = usize;

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            game: None, 
            settings: Settings::default(),
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

                if let Some(mut player_name) = self.player_names.get_mut(player_index){
                    player_name = &mut (send.clone(), name.clone());
                }
                let mut name = name.trim().to_string();
                // if name.len() == 0{
                //     name
                // }
                send.send(ToClientPacket::YourName{name});
            },
            ToServerPacket::StartGame => {
                    
                if(self.game.is_none()){
                    self.game = Some(Game::new(self.settings.clone(), self.player_names.clone()));
                }

                for player in self.player_names.iter(){
                    player.0.send(ToClientPacket::OpenGameMenu);
                }
            },
            ToServerPacket::Kick{player_index} => {
                
            },
            ToServerPacket::SetRoleList{role_list} => todo!(),
            ToServerPacket::SetPhaseTimes{phase_times} => todo!(),
            ToServerPacket::SetInvestigatorResults{investigator_results} => todo!(),
            _ => {
                if self.game.is_some(){ //TODO jack please jack help please jack plz
                    self.game.as_mut().unwrap().on_client_message(player_index, incoming_packet);
                }
            }
        }
    }
}