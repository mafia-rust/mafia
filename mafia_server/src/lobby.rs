use std::{collections::HashMap, net::SocketAddr, fs, default};

use serde::__private::de::{Content, self};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::{PlayerIndex, Player}, settings::{Settings, InvestigatorResults}, role_list}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}, utils::trim_whitespace};

pub struct Lobby {
    game: Option<Game>,
    settings: Settings,
    player_names: Vec<(UnboundedSender<ToClientPacket>, String)>,

    random_names: Vec<String>
}

pub type LobbyIndex = usize;

impl Lobby {
    pub fn new() -> Lobby {

        //TODO it crashes and also loads the file every time a new lobby is made this is obviously bad
        let mut default_names: Vec<String> = 
            fs::read_to_string(".\\resources\\random_names\\default_names.csv").expect("Should have been able to read the file")
            .split("\r\n").map(|s|{s.to_string()}).collect();
        let mut extra_names: Vec<String> = 
            fs::read_to_string(".\\resources\\random_names\\extra_names.csv").expect("Should have been able to read the file")
            .split("\r\n").map(|s|{s.to_string()}).collect();

        let mut random_names = Vec::new();
        random_names.append(&mut default_names);
        random_names.append(&mut extra_names);

        Self { 
            game: None,
            settings: Settings::default(),
            player_names: Vec::new(),
            random_names,
        }
    }
    pub fn add_new_player(&mut self, sender: UnboundedSender<ToClientPacket>)->PlayerIndex{
        self.player_names.push((sender.clone(),"".to_owned()));
        let newest_player_index = self.player_names.len() - 1;
        self.simulate_message(newest_player_index, ToServerPacket::SetName { name: "".to_string() });
        sender.send(ToClientPacket::Players { names: self.player_names.iter().map(|p|{
            p.1.clone()
        }).collect() });
        newest_player_index
    }
    pub fn simulate_message(&mut self, player_sender: PlayerIndex, packet: ToServerPacket){
        self.on_client_message(self.player_names[player_sender].0.clone(), player_sender, packet);
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<ToClientPacket>, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                
                //fix it up
                let mut name = trim_whitespace(name.trim());

                //if its invlaid then give you a random name
                if name.len() == 0 {

                    let availabe_random_names: Vec<&String> = self.random_names.iter().filter(|name|{
                        !self.player_names.iter().map(|(_,n)|{
                            n.clone()
                        }).collect::<Vec<String>>().contains(name)
                    }).collect();

                    //If there are 32 players in the lobby this will crash TODO
                    if availabe_random_names.len() > 0{
                        panic!("RAN OUT OF NAMES")
                    }else{
                        name = availabe_random_names[rand::random::<usize>()%availabe_random_names.len()].clone();
                    }
                }


                if let Some(mut player_name) = self.player_names.get_mut(player_index){
                    player_name.1 = name.clone();
                }
                
                send.send(ToClientPacket::YourName{name});
                for (sender, _) in self.player_names.iter(){
                    sender.send(ToClientPacket::Players { names: self.player_names.iter().map(|p|{
                        p.1.clone()
                    }).collect() });
                }
                
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
            ToServerPacket::SetPhaseTimes{phase_times} => {
                self.settings.phase_times = phase_times.clone();

                send.send(ToClientPacket::PhaseTimes { phase_times });
            },
            ToServerPacket::SetInvestigatorResults{investigator_results} => todo!(),
            _ => {
                if let Some(game) = &mut self.game{
                    game.on_client_message(player_index, incoming_packet)
                }
            }
        }
    }
}