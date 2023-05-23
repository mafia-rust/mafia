use std::{collections::HashMap, net::SocketAddr, fs, time::Duration, hash::Hash};

use futures_util::pending;
use lazy_static::lazy_static;
use serde::__private::de::{Content, self};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    game::{
        Game, 
        player::{PlayerIndex, Player, PlayerReference}, 
        settings::{Settings, self}, 
        role_list::{self, RoleList, RoleListEntry}, 
        phase::PhaseType
    },
    utils::trim_whitespace, log, listener::ArbitraryPlayerID, packet::{ToClientPacket, RejectJoinReason, ToServerPacket, RejectStartReason}
};

pub struct Lobby {
    lobby_state: LobbyState,
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

#[derive(Clone)]
pub struct LobbyPlayer{
    pub sender: UnboundedSender<ToClientPacket>,
    pub name: String,
}

lazy_static!(
    static ref RANDOM_NAMES: Vec<String> = {
        let mut random_names = Vec::new();
        random_names.append(&mut 
            include_str!("../resources/random_names/default_names.csv").lines()
                .map(str::to_string)
                .collect()
        );
        random_names.append(&mut 
            include_str!("../resources/random_names/extra_names.csv").lines()
                .map(str::to_string)
                .collect()
        );

        random_names
    };
);

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                players: HashMap::new()
            }
        }
    }

    /// Catches the sender up with the current lobby settings
    pub fn inform_player(sender: UnboundedSender<ToClientPacket>, settings: &Settings) {
        for phase in [
            PhaseType::Discussion, 
            PhaseType::Evening, 
            PhaseType::Judgement, 
            PhaseType::Morning,
            PhaseType::Night,
            PhaseType::Testimony,
            PhaseType::Voting
        ] {
            sender.send(ToClientPacket::PhaseTime { 
                phase, 
                time: settings.phase_times.get_time_for(phase).as_secs() 
            });
        }
        sender.send(ToClientPacket::RoleList { role_list: settings.role_list.clone() });
    }

    pub fn join_player(&mut self, sender: UnboundedSender<ToClientPacket>)-> Result<ArbitraryPlayerID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { players, settings } => {
                // TODO, move this somewhere else
                let name = Self::validate_name(players, "".to_string());
                
                sender.send(ToClientPacket::YourName { name: name.clone() });
                // Add a role list entry
                settings.role_list.push(RoleListEntry::Any);
                Self::inform_player(sender.clone(), settings);
                
                let arbitrary_player_id = players.len() as ArbitraryPlayerID;

                let player = LobbyPlayer {
                    sender,
                    name,
                };

                players.insert(arbitrary_player_id, player);

                let role_list = settings.role_list.clone();

                // Make sure everybody is on the same page
                self.send_players();
                self.send_to_all(ToClientPacket::RoleList { role_list });

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

                let mut other_players = players.clone();
                other_players.remove(&player_arbitrary_id);
                
                let name = Self::validate_name(&other_players, name.clone());
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

    fn validate_name(players: &HashMap<ArbitraryPlayerID, LobbyPlayer>, mut name: String) -> String {
        name = trim_whitespace(name.trim());
        name.truncate(30);

        //if valid then return
        if name.len() > 0 && !players.values()
            .any(|existing_player| name == *existing_player.name)
        {
            return name;
        }
        drop(name);

        //otherwise 
        let available_random_names: Vec<&String> = RANDOM_NAMES.iter().filter(|new_random_name| {
            !players.values()
                .map(|p| &p.name)
                .any(|existing_name|{
                        let mut new_random_name = trim_whitespace(new_random_name.trim());
                        new_random_name.truncate(30);

                        
                        let mut existing_name = trim_whitespace(existing_name.trim());
                        existing_name.truncate(30);

                        new_random_name == existing_name
                    }
                )
        }).collect();

        if available_random_names.len() > 0 {
            return available_random_names[rand::random::<usize>()%available_random_names.len()].clone();
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
                names: PlayerReference::all_players(game).iter().map(|p| {
                    p.name(game).clone()
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
                for player_ref in PlayerReference::all_players(game){
                    player_ref.send_packet(game, packet.clone());
                }
            }
            LobbyState::Closed => {}
        }
    }
}