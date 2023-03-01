use std::{collections::HashMap, net::SocketAddr, fs};

use serde::__private::de::{Content, self};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{game::{Game, player::{PlayerIndex, Player}, settings::{Settings, InvestigatorResults}, role_list}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket}}, utils::trim_whitespace};

pub struct Lobby {
    lobby_state: LobbyState,

    random_names: Vec<String>
}
enum LobbyState{
    Lobby{
        settings: Settings,
        players: Vec<(UnboundedSender<ToClientPacket>, String)>,
    },
    Game(Game)
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
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                players: Vec::new()
            }, 
            random_names
        }
    }
    pub fn add_new_player(&mut self, sender: UnboundedSender<ToClientPacket>)->PlayerIndex{

        match &mut self.lobby_state {
            LobbyState::Lobby { settings, players } => {
                players.push((sender.clone(),"".to_owned()));

                //////////////////////////////////
                let availabe_random_names: Vec<&String> = self.random_names.iter().filter(|name|{
                    !players.iter().map(|(_,n)|{
                        n.clone()
                    }).collect::<Vec<String>>().contains(name)
                }).collect();

                //If there are 32 players in the lobby this will crash TODO
                if availabe_random_names.len() > 0{
                    todo!("RAN OUT OF NAMES")
                }else{
                    let name = availabe_random_names[rand::random::<usize>()%availabe_random_names.len()].clone();
                    sender.send(ToClientPacket::YourName{name});
                }
                ////////////////////////////////////////


                sender.send(ToClientPacket::Players { names: players.iter().map(|p|{
                    p.1.clone()
                }).collect() });

                let newest_player_index = (players.len() - 1) as u8;
                newest_player_index
            },
            LobbyState::Game(game) => {
                todo!()
            },
        }
        

    }
    pub fn on_client_message(&mut self, send: UnboundedSender<ToClientPacket>, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                if let LobbyState::Lobby { settings, players } = &mut self.lobby_state{
                    //fix it up
                    let mut name = trim_whitespace(name.trim());
                    //TODO make names max length 30

                    //if its invlaid then give you a random name
                    if name.len() == 0 {

                        let availabe_random_names: Vec<&String> = self.random_names.iter().filter(|name|{
                            !players.iter().map(|(_,n)|{
                                n.clone()
                            }).collect::<Vec<String>>().contains(name)
                        }).collect();

                        //If there are 32 players in the lobby this will crash TODO
                        if availabe_random_names.len() > 0{
                            todo!("RAN OUT OF NAMES")
                        }else{
                            name = availabe_random_names[rand::random::<usize>()%availabe_random_names.len()].clone();
                        }
                    }


                    if let Some(mut player_name) = players.get_mut(player_index as usize){
                        player_name.1 = name.clone();
                    }
                    
                    send.send(ToClientPacket::YourName{name});
                    for (sender, _) in players.iter(){
                        sender.send(ToClientPacket::Players { names: players.iter().map(|p|{
                            p.1.clone()
                        }).collect() });
                    }
                }               
            },
            ToServerPacket::StartGame => {           
                if let LobbyState::Lobby { settings, players } = &mut self.lobby_state{
                    
                    for player in players.iter(){
                        player.0.send(ToClientPacket::OpenGameMenu);
                    }

                    //why does it let me do this?? double borrow of lobby_state
                    self.lobby_state = LobbyState::Game(Game::new(settings.clone(), players.clone()))
                }
            },
            ToServerPacket::Kick{player_index} => {
                todo!()// cant kick because then all player_index's would need to change and all players would be pointing to the wrong indeices
            },
            ToServerPacket::SetRoleList{role_list} => todo!(),
            ToServerPacket::SetPhaseTimes{phase_times} => {
                if let LobbyState::Lobby{ settings, players } = &mut self.lobby_state{
                    settings.phase_times = phase_times.clone();
                    send.send(ToClientPacket::PhaseTimes { phase_times });
                }
            },
            ToServerPacket::SetInvestigatorResults{investigator_results} => todo!(),
            _ => {
                if let LobbyState::Game(game) = &mut self.lobby_state{
                    game.on_client_message(player_index, incoming_packet)
                }
            }
        }
    }
}