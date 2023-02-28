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
        self.player_names.push((sender,"".to_owned()));
        let newest_player_index = self.player_names.len() - 1;
        self.simulate_message(newest_player_index, ToServerPacket::SetName { name: "".to_string() });
        newest_player_index
    }
    pub fn simulate_message(&mut self, player_sender: PlayerIndex, packet: ToServerPacket){
        self.on_client_message(self.player_names[player_sender].0.clone(), player_sender, packet);
        
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<ToClientPacket>, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {

                //define recusive closure
                struct MakeValid<'s> { func: &'s dyn Fn(&MakeValid, String) -> String }
                let make_valid = MakeValid {
                    func: &|make_valid, mut name|{
                        
                        //fix it up
                        name = trim_whitespace(name.trim());

                        //if its empty then give you a random name
                        if name.len() == 0 {
                            name = self.random_names[rand::random::<usize>()%self.random_names.len()].clone();
                        }

                        //check if someone else has the same
                        let mut taken = true;
                        for (_, other_name) in self.player_names.iter(){
                            if name == *other_name{
                                taken = false;
                                break;
                            }
                        }
                        
                        if !taken{
                            name = (make_valid.func)(make_valid, name);
                        }
                        return name;
                    }
                };

                let name = (make_valid.func)(&make_valid, name);

                if let Some(mut player_name) = self.player_names.get_mut(player_index){
                    player_name = &mut (send.clone(), name.clone());
                }
                
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