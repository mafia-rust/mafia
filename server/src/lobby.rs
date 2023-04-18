use std::{collections::HashMap, net::SocketAddr, fs, time::Duration, hash::Hash};

use futures_util::pending;
use serde::__private::de::{Content, self};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    game::{Game, player::{PlayerIndex, Player}, 
    settings::{Settings, InvestigatorResults, self}, 
    role_list, phase::PhaseType}, network::{connection::Connection, packet::{ToServerPacket, ToClientPacket, RejectJoinReason, RejectStartReason}, listener::ArbitraryPlayerID}, 
    utils::trim_whitespace
};

pub struct Lobby {
    lobby_state: LobbyState,
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

                let player = LobbyPlayer{
                    sender,
                    name: name,
                };                
                
                
                let newest_player_arbitrary_id = players.len() as ArbitraryPlayerID;

                players.insert(
                    newest_player_arbitrary_id,  
                    player
                );


                Self::send_players(players);

                Ok(newest_player_arbitrary_id)
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
                if let LobbyState::Lobby { settings, players } = &mut self.lobby_state{

                    let name = Self::validate_name(&self.random_names, players, name.clone());
                    if let Some(mut player) = players.get_mut(&player_arbitrary_id){
                        player.name = name.clone();
                    }
                    
                    send.send(ToClientPacket::YourName{name});

                    Self::send_players(players);
                }
            },
            ToServerPacket::StartGame => {
                if let LobbyState::Lobby { settings, players } = &mut self.lobby_state{
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
                    
                    for (_, player) in players.iter(){
                        player.sender.send(ToClientPacket::OpenGameMenu);
                    }


                    let mut player_indices: HashMap<ArbitraryPlayerID,PlayerIndex> = HashMap::new();
                    let mut game_players = Vec::new();
                    
                    let mut i = 0;
                    for (a_id, lobby_player) in players.drain() {

                        player_indices.insert(a_id, i);
                        game_players.push(lobby_player);
                        i+=1;
                    }

                    self.lobby_state = LobbyState::Game{
                        game: Game::new(settings.clone(), game_players),
                        players: player_indices,
                    }
                }
            },
            ToServerPacket::Kick{player_index} => {
                todo!()// cant kick because then all player_index's would need to change and all players would be pointing to the wrong indeices
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                if let LobbyState::Lobby{ settings, players } = &mut self.lobby_state{
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
                    
                    Self::send_to_all(players, ToClientPacket::PhaseTime { phase, time });
                }
            },
            ToServerPacket::SetRoleList { role_list } => {
                if let LobbyState::Lobby{ settings, players } = &mut self.lobby_state{
                    settings.role_list = role_list.clone();
                    
                    Self::send_to_all(players, ToClientPacket::RoleList { role_list });
                }
            }
            ToServerPacket::SetInvestigatorResults{investigator_results} => todo!(),
            _ => {
                if let LobbyState::Game { game, players } = &mut self.lobby_state{
                    game.on_client_message(players.get(&player_arbitrary_id).unwrap().clone(), incoming_packet)
                }
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
    fn send_players(players: &mut HashMap<ArbitraryPlayerID, LobbyPlayer>){
        for (_, player) in players.iter(){
            player.sender.send(ToClientPacket::Players { names: players.iter().map(|p|{
                p.1.name.clone()
            }).collect() });
        }
    }
    fn send_to_all(players: &mut HashMap<ArbitraryPlayerID, LobbyPlayer>, packet: ToClientPacket){
        for player in players.iter(){
            player.1.sender.send(packet.clone());
        }
    }
}