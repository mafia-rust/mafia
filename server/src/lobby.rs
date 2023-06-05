use std::{collections::HashMap, time::Duration};

use crate::{
    game::{
        Game, 
        player::{PlayerIndex, PlayerReference}, 
        settings::Settings, 
        role_list::RoleListEntry, 
        phase::PhaseType
    },
    log, listener::ArbitraryPlayerID, packet::{ToClientPacket, RejectJoinReason, ToServerPacket, RejectStartReason}, websocket_connections::connection::{Connection, ClientSender}
};

pub struct Lobby {
    lobby_state: LobbyState,
}

enum LobbyState {
    Lobby {
        settings: Settings,
        players: HashMap<ArbitraryPlayerID, LobbyPlayer>,
    },
    Game {
        game: Game,
        players: HashMap<ArbitraryPlayerID, PlayerIndex>,
    },
    Closed
}

#[derive(Clone)]
pub struct LobbyPlayer{
    pub sender: ClientSender,
    pub name: String,
}

impl LobbyPlayer {
    pub fn send(&self, message: ToClientPacket) {
        self.sender.send(message);
    }
}

impl Lobby {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Lobby {
        Self { 
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                players: HashMap::new()
            }
        }
    }

    
    pub fn on_client_message(&mut self, send: &Connection, player_arbitrary_id: ArbitraryPlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { players, .. } = &mut self.lobby_state else {
                    println!("{} {}", log::error("ToServerPacket::SetName can not be used outside of LobbyState::Lobby"), player_arbitrary_id);
                    return;
                };

                let mut other_players = players.clone();
                other_players.remove(&player_arbitrary_id);
                
                let name = name_validation::sanitize_name(name, &other_players);
                if let Some(mut player) = players.get_mut(&player_arbitrary_id){
                    player.name = name.clone();
                }
                
                send.send(ToClientPacket::YourName{name});

                Self::send_players_lobby(players);
            },
            ToServerPacket::StartGame => {
                let LobbyState::Lobby { settings, players } = &mut self.lobby_state else {
                    println!("{} {}", log::error("ToServerPacket::StartGame can not be used outside of LobbyState::Lobby"), player_arbitrary_id);
                    return;
                };
                
                if [
                    settings.phase_times.evening, settings.phase_times.morning,
                    settings.phase_times.discussion, settings.phase_times.voting,
                    settings.phase_times.judgement, settings.phase_times.testimony,
                    settings.phase_times.night
                ].iter().all(Duration::is_zero) {
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
            ToServerPacket::Kick{..} => {
                //TODO
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                let LobbyState::Lobby{ settings, .. } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Attempted to change phase time outside of the lobby menu!"), player_arbitrary_id);
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
                let LobbyState::Lobby{ settings, .. } = &mut self.lobby_state else {
                    println!("{} {}", log::error("Can't modify game settings outside of the lobby menu"), player_arbitrary_id);
                    return;
                };
                settings.role_list = role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            _ => {
                let LobbyState::Game { game, players } = &mut self.lobby_state else {
                    println!("{} {:?}", log::error("ToServerPacket not implemented for lobby was sent during lobby: "), incoming_packet);
                    return;
                };

                game.on_client_message(players[&player_arbitrary_id], incoming_packet)
            }
        }
    }

    pub fn connect_player_to_lobby(&mut self, send: &ClientSender)-> Result<ArbitraryPlayerID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { players, settings } => {
                // TODO, move this somewhere else
                let name = name_validation::sanitize_name("".to_string(), players);
                
                send.send(ToClientPacket::YourName { name: name.clone() });
                
                let new_player = LobbyPlayer { sender: send.clone(), name };
                let arbitrary_player_id = players.len() as ArbitraryPlayerID;
                players.insert(arbitrary_player_id, new_player);

                settings.role_list.push(RoleListEntry::Any);

                // Make sure everybody is on the same page
                Self::send_players_lobby(players);
                for player in players.iter(){
                    Self::send_settings(player.1, settings)
                }
                send.send(ToClientPacket::AcceptJoin{in_game: false});
                Ok(arbitrary_player_id)
            },
            LobbyState::Game{ game, players } => {

                for player_ref in PlayerReference::all_players(&game){
                    if !player_ref.is_connected(&game){
                        //loop through all arbitrary player ids and find the first one that isn't connected
                        let arbitrary_player_ids: Vec<ArbitraryPlayerID> = players.iter().map(|(id, _)|*id).collect();
                        let mut new_id: ArbitraryPlayerID = 0;
                        for id in arbitrary_player_ids {
                            if id >= new_id {
                                new_id = id + 1;
                            }
                        }

                        players.insert(new_id, player_ref.index());
                        player_ref.connect(game, send.clone());
                        
                        send.send(ToClientPacket::AcceptJoin{in_game: true});
                        
                        game.send_start_game_information(player_ref);
                        
                        return Ok(new_id);
                    }
                }

                Err(RejectJoinReason::GameAlreadyStarted)
            }
            LobbyState::Closed => {
                Err(RejectJoinReason::InvalidRoomCode)
            }
        }
    }
    pub fn disconnect_player_from_lobby(&mut self, id: ArbitraryPlayerID) {
        match &mut self.lobby_state {
            LobbyState::Lobby {players, settings} => {
                players.remove(&id);
            
                if players.is_empty() {
                    self.lobby_state = LobbyState::Closed;
                    return;
                }

                settings.role_list.pop();

                Self::send_players_lobby(players);
                for player in players.iter(){
                    Self::send_settings(player.1, settings)
                }
            },
            LobbyState::Game {game, players} => {
                // game.on_client_disconnect(*players.get(&id).unwrap());
                //TODO proper disconnect from game
                let player_index = players.get(&id);
                if let Some(player_index) = player_index {
                    if let Ok(player_ref) = PlayerReference::new(game, *player_index) {
                        player_ref.disconnect(game);
                    }
                }
                players.remove(&id);
            },
            LobbyState::Closed => {}
        }
    }
    
    pub fn is_closed(&self) -> bool {
        matches!(self.lobby_state, LobbyState::Closed)
    }

    pub fn tick(&mut self, time_passed: Duration){
        if let LobbyState::Game { game, .. } = &mut self.lobby_state {
            game.tick(time_passed);
        }
    }
    
    /// Catches the sender up with the current lobby settings
    pub fn send_settings(player: &LobbyPlayer, settings: &Settings) {
        for phase in [
            PhaseType::Morning,
            PhaseType::Discussion, 
            PhaseType::Voting,
            PhaseType::Testimony,
            PhaseType::Judgement,
            PhaseType::Evening, 
            PhaseType::Night,
        ] {
            player.send(ToClientPacket::PhaseTime { 
                phase, 
                time: settings.phase_times.get_time_for(phase).as_secs() 
            });
        }
        player.send(ToClientPacket::RoleList { role_list: settings.role_list.clone() });
    }


    fn send_players_lobby(players: &HashMap<ArbitraryPlayerID, LobbyPlayer>){
        let packet = ToClientPacket::Players { 
            names: players.iter().map(|p| {
                p.1.name.clone()
            }).collect() 
        };
        for player in players.iter() {
            player.1.send(packet.clone());
        }
    }
    #[allow(unused)]
    fn send_players_game(game: &mut Game, players: &HashMap<ArbitraryPlayerID, PlayerIndex>){
        let packet = ToClientPacket::Players { 
            names: PlayerReference::all_players(game).into_iter().map(|p| {
                p.name(game).clone()
            }).collect() 
        };
        for player_ref in PlayerReference::all_players(game){
            player_ref.send_packet(game, packet.clone());
        }
    }

    fn send_to_all(&mut self, packet: ToClientPacket){
        match &mut self.lobby_state {
            LobbyState::Lobby { players, .. } => {
                for player in players.iter() {
                    player.1.send(packet.clone());
                }
            }
            LobbyState::Game { game, .. } => {
                for player_ref in PlayerReference::all_players(game){
                    player_ref.send_packet(game, packet.clone());
                }
            }
            LobbyState::Closed => {}
        }
    }
}

mod name_validation {
    use std::collections::HashMap;
    use crate::{listener::ArbitraryPlayerID, strings::TidyableString};
    use super::LobbyPlayer;
    use lazy_static::lazy_static;
    use rand::seq::SliceRandom;

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

    const MAX_NAME_LENGTH: usize = 20;

    /// Sanitizes a player name.
    /// If the desired name is invalid or taken, this generates a random acceptable name.
    /// Otherwise, this trims and returns the input name.
    pub fn sanitize_name(mut desired_name: String, players: &HashMap<ArbitraryPlayerID, LobbyPlayer>) -> String {
        desired_name = desired_name.trim().to_string()
            .trim_newline()
            .trim_whitespace()
            .truncate(MAX_NAME_LENGTH);
    
        let name_already_taken = players.values().any(|existing_player| desired_name == *existing_player.name);
        
        if !desired_name.is_empty() && !name_already_taken {
            desired_name
        } else {
            generate_random_name(&players.values().map(|p| p.name.as_str()).collect::<Vec<&str>>())
        }
    }

    pub fn generate_random_name(taken_names: &[&str]) -> String{
        let available_random_names = RANDOM_NAMES.iter().filter(|new_random_name| {
            !taken_names.iter()
                .any(|existing_name| {
                    let new_random_name = new_random_name.trim().to_string()
                        .trim_newline()
                        .trim_whitespace()
                        .truncate(MAX_NAME_LENGTH);

                    let existing_name = existing_name.trim().to_string()
                        .trim_newline()
                        .trim_whitespace()
                        .truncate(MAX_NAME_LENGTH);

                    new_random_name == existing_name
                })
        }).collect::<Vec<&String>>();
    
        if let Some(random_name) = available_random_names.choose(&mut rand::thread_rng()) {
            (*random_name).clone()
        } else {
            // This name generator sucks, but at least it works.
            (taken_names.len()).to_string()
        }
    }
}
