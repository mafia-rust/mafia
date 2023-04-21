use std::{collections::HashMap, net::SocketAddr, fs, time::Duration, hash::Hash};

use futures_util::pending;
use serde::__private::de::{Content, self};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    game::{Game, player::{PlayerIndex, Player}, 
    settings::{Settings, InvestigatorResults, self}, 
    role_list, phase::PhaseType}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket, RejectJoinReason, RejectStartReason}, listener::ArbitraryPlayerID}, 
    utils::trim_whitespace, log
};

pub struct Lobby {
    lobby_state: LobbyState,
    // TODO remove
    random_names: Vec<String>,
}

enum LobbyState{
    Lobby{
        settings: Settings,
        players: HashMap<ArbitraryPlayerID, LobbyPlayer>,
    },
    Game{
        game: Game,
        players: HashMap<ArbitraryPlayerID, PlayerIndex>,
    },
    Closed
}
pub struct LobbyPlayer{
    pub sender: UnboundedSender<ToClientPacket>,
    pub name: String,
}

impl Lobby {
    pub fn new() -> Lobby {

        //TODO it crashes and also loads the file every time a new lobby is made this is obviously bad
        let mut default_names: Vec<String> = 
            fs::read_to_string("./resources/random_names/default_names.csv").expect("Should have been able to read the file").lines()
            .map(|s|{s.to_string()}).collect();
        let mut extra_names: Vec<String> = 
            fs::read_to_string("./resources/random_names/extra_names.csv").expect("Should have been able to read the file").lines()
            .map(|s|{s.to_string()}).collect();

        let mut random_names = Vec::new();
        random_names.append(&mut default_names);
        random_names.append(&mut extra_names);

        

        Self { 
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                players: HashMap::new()
            },
            random_names,
        }
    }
    pub fn join_player(&mut self, sender: UnboundedSender<ToClientPacket>)-> Result<ArbitraryPlayerID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { players, .. } => {
                // TODO, move this somewhere else
                let name = Self::validate_name(&self.random_names, players, "".to_string());
                
                sender.send(ToClientPacket::YourName { name: name.clone() });
                // TODO "Catch player up" on lobby settings
                
                let arbitrary_player_id = players.len() as ArbitraryPlayerID;

                let player = LobbyPlayer {
                    sender,
                    name,
                };

                players.insert(arbitrary_player_id, player);

                self.send_players();

                Ok(arbitrary_player_id)
            },
            LobbyState::Game{ .. } => {
                // TODO, handle rejoining
                Err(RejectJoinReason::GameAlreadyStarted)
            }
            LobbyState::Closed => {
                Err(RejectJoinReason::InvalidRoomCode)
            }
        }
    }
    pub fn disconnect_player(&mut self, id: ArbitraryPlayerID) {
        if let LobbyState::Lobby { ref mut players, .. } = &mut self.lobby_state {
            players.remove(&id);

            if players.len() == 0 {
                self.lobby_state = LobbyState::Closed;
            }
        }
    }
    pub fn on_client_message(&mut self, send: UnboundedSender<ToClientPacket>, player_arbitrary_id: ArbitraryPlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { settings, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Player tried to set name before joining a lobby!"), player_arbitrary_id);
                    return;
                };

                let name = Self::validate_name(&self.random_names, players, name.clone());
                if let Some(mut player) = players.get_mut(&player_arbitrary_id){
                    player.name = name.clone();
                }
                
                send.send(ToClientPacket::YourName{name});

                self.send_players();
            },
            ToServerPacket::StartGame => {
                let LobbyState::Lobby { settings, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Player tried to start game before joining a lobby!"), player_arbitrary_id);
                    return;
                };
                
                if (settings.phase_times.evening.is_zero() &&
                    settings.phase_times.morning.is_zero() &&
                    settings.phase_times.discussion.is_zero() &&
                    settings.phase_times.voting.is_zero() &&
                    settings.phase_times.judgement.is_zero() &&
                    settings.phase_times.testimony.is_zero() &&
                    settings.phase_times.night.is_zero()
                ) {
                    send.send(ToClientPacket::RejectStart { reason: RejectStartReason::ZeroTimeGame });
                    return;
                }

                let mut player_indices: HashMap<ArbitraryPlayerID,PlayerIndex> = HashMap::new();
                let mut game_players = Vec::new();
                
                for (index, (arbitrary_player_id, lobby_player)) in players.drain().enumerate() {
                    player_indices.insert(arbitrary_player_id, index as u8);
                    game_players.push(lobby_player);
                }

                self.lobby_state = LobbyState::Game{
                    game: Game::new(settings.clone(), game_players),
                    players: player_indices,
                };
                
                self.send_to_all(ToClientPacket::StartGame);
            },
            ToServerPacket::Kick{player_index} => {
                todo!()// cant kick because then all player_index's would need to change and all players would be pointing to the wrong indeices
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Player tried to set phase time settings before joining a lobby!"), player_arbitrary_id);
                    return;
                };

                let phase_time = Duration::from_secs(time);

                match phase {
                    PhaseType::Morning => { settings.phase_times.morning = phase_time; }
                    PhaseType::Discussion => { settings.phase_times.discussion = phase_time; }
                    PhaseType::Evening => { settings.phase_times.evening = phase_time; }
                    PhaseType::Judgement => { settings.phase_times.judgement = phase_time; }
                    PhaseType::Night => { settings.phase_times.night = phase_time; }
                    PhaseType::Testimony => { settings.phase_times.testimony = phase_time; }
                    PhaseType::Voting => { settings.phase_times.voting = phase_time; }
                };
                
                self.send_to_all(ToClientPacket::PhaseTime { phase, time });
            },
            ToServerPacket::SetRoleList { role_list } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Player tried to set role list settings before joining a lobby!"), player_arbitrary_id);
                    return;
                };
                settings.role_list = role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetInvestigatorResults{investigator_results} => todo!(),
            _ => {
                let LobbyState::Game { game, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Player tried to set investigator results settings before joining a lobby!"), player_arbitrary_id);
                    return;
                };

                game.on_client_message(players.get(&player_arbitrary_id).unwrap().clone(), incoming_packet)
            }
        }
    }

    pub fn is_closed(&self) -> bool {
        match &self.lobby_state {
            LobbyState::Closed => true,
            _ => false
        }
    }

    pub fn tick(&mut self, time_passed: Duration){
        if let LobbyState::Game { game, players } = &mut self.lobby_state {
            game.tick(time_passed);
        }
    }

    fn validate_name(random_names: &Vec<String>, players: &mut HashMap<ArbitraryPlayerID, LobbyPlayer>, mut name: String)->String{

        name = trim_whitespace(name.trim());

        if name.len() > 0 {
            return name;
        }

        let availabe_random_names: Vec<&String> = random_names.iter().filter(|name|{
        
            !players.iter().map(|(_,n)|{
                n.name.clone()
            }).collect::<Vec<String>>().contains(name)
        
        }).collect();

        if availabe_random_names.len() > 0 {
            return availabe_random_names[rand::random::<usize>()%availabe_random_names.len()].clone();
        } else {
            // Awesome name generator
            // TODO make this better, or don't.
            return players.len().to_string()
        }
    }
    fn send_players(&self){
        let packet = match &self.lobby_state {
            LobbyState::Lobby { players, .. } => ToClientPacket::Players { 
                names: players.iter().map(|p| {
                    p.1.name.clone()
                }).collect() 
            },
            LobbyState::Game { game, players } => ToClientPacket::Players { 
                names: players.iter().map(|p| {
                    game.get_player(*p.1).unwrap().name.clone()
                }).collect() 
            },
            LobbyState::Closed => {return;}
        };
        self.send_to_all(packet);
    }
    fn send_to_all(&self, packet: ToClientPacket){
        match &self.lobby_state {
            LobbyState::Lobby { players, .. } => {
                for player in players.iter() {
                    player.1.sender.send(packet.clone());
                }
            }
            LobbyState::Game { game, players } => {
                for player in players.iter() {
                    game.get_unchecked_player(*player.1).send(packet.clone());
                }
            }
            LobbyState::Closed => {}
        }
    }
}