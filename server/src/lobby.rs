use std::{collections::{HashMap, HashSet}, time::Duration};

use crate::{
    game::{
        Game, 
        player::{PlayerIndex, PlayerReference}, 
        settings::Settings, 
        role_list::RoleListEntry, 
        phase::PhaseType
    },
    listener::ArbitraryPlayerID, packet::{ToClientPacket, RejectJoinReason, ToServerPacket, RejectStartReason}, websocket_connections::connection::{ClientSender}, log
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
        players: HashMap<ArbitraryPlayerID, GamePlayer>,
    },
    Closed
}

#[derive(Clone)]
pub struct LobbyPlayer{
    pub sender: ClientSender,
    pub name: String,
    pub host: bool
}

impl LobbyPlayer {
    pub fn set_host(&mut self) {
        self.host = true;
        self.sender.send(ToClientPacket::YouAreHost);
    }

    pub fn send(&self, message: ToClientPacket) {
        self.sender.send(message);
    }
}
#[derive(Clone)]
pub struct GamePlayer{
    pub player_index: PlayerIndex,
    pub host: bool
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

    
    pub fn on_client_message(&mut self, send: &ClientSender, player_arbitrary_id: ArbitraryPlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { players, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", player_arbitrary_id);
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
                    log!(error "Lobby"; "{} {}", "ToServerPacket::StartGame can not be used outside of LobbyState::Lobby", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }
                
                if [
                    settings.phase_times.evening, settings.phase_times.morning,
                    settings.phase_times.discussion, settings.phase_times.voting,
                    settings.phase_times.judgement, settings.phase_times.testimony,
                    settings.phase_times.night,
                ].iter().all(|t| *t == 0) {
                    send.send(ToClientPacket::RejectStart { reason: RejectStartReason::ZeroTimeGame });
                    return;
                }

                let mut player_indices: HashMap<ArbitraryPlayerID,GamePlayer> = HashMap::new();
                let mut game_players = Vec::new();
                
                for (index, (arbitrary_player_id, lobby_player)) in players.drain().enumerate() {
                    player_indices.insert(arbitrary_player_id, GamePlayer { player_index: index as PlayerIndex, host: lobby_player.host });
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
                let LobbyState::Lobby{ settings, players  } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }

                match phase {
                    PhaseType::Morning => { settings.phase_times.morning = time; }
                    PhaseType::Discussion => { settings.phase_times.discussion = time; }
                    PhaseType::Evening => { settings.phase_times.evening = time; }
                    PhaseType::Judgement => { settings.phase_times.judgement = time; }
                    PhaseType::Night => { settings.phase_times.night = time; }
                    PhaseType::Testimony => { settings.phase_times.testimony = time; }
                    PhaseType::Voting => { settings.phase_times.voting = time; }
                };
                
                self.send_to_all(ToClientPacket::PhaseTime { phase, time });
            },
            ToServerPacket::SetPhaseTimes { phase_time_settings } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }

                settings.phase_times = phase_time_settings.clone();

                self.send_to_all(ToClientPacket::PhaseTimes { phase_time_settings });
            }
            ToServerPacket::SetRoleList { mut role_list } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }


                while role_list.len() < players.len() {
                    role_list.push(RoleListEntry::Any);
                }
                while players.len() < role_list.len() {
                    role_list.pop();
                }
                settings.role_list = role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetRoleListEntry { index, role_list_entry } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }

                settings.role_list[index as usize] = role_list_entry.clone();
                
                self.send_to_all(ToClientPacket::RoleListEntry { index, role_list_entry });
            }
            ToServerPacket::SetExcludedRoles {mut roles } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_arbitrary_id);
                    return;
                };
                if let Some(player) = players.get(&player_arbitrary_id){
                    if !player.host {return;}
                }


                let roles: HashSet<_> = roles.drain(..).collect();
                let roles: Vec<_> = roles.into_iter().collect();
                settings.excluded_roles = roles.clone();
                self.send_to_all(ToClientPacket::ExcludedRoles { roles });
            }
            _ => {
                let LobbyState::Game { game, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {:?}", "ToServerPacket not implemented for lobby was sent during lobby: ", incoming_packet);
                    return;
                };

                game.on_client_message(players[&player_arbitrary_id].player_index, incoming_packet)
            }
        }
    }

    pub fn connect_player_to_lobby(&mut self, send: &ClientSender)-> Result<ArbitraryPlayerID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { players, settings } => {
                let name = name_validation::sanitize_name("".to_string(), players);
                
                send.send(ToClientPacket::YourName { name: name.clone() });
                
                let new_player = LobbyPlayer { name, sender: send.clone(), host: players.is_empty() };
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

                for player_ref in PlayerReference::all_players(game){
                    if player_ref.has_lost_connection(game) {
                        //loop through all arbitrary player ids and find the first one that isn't connected
                        let arbitrary_player_ids: Vec<ArbitraryPlayerID> = players.iter().map(|(id, _)|*id).collect();
                        let mut new_id: ArbitraryPlayerID = 0;
                        for id in arbitrary_player_ids {
                            if id >= new_id {
                                new_id = id + 1;
                            }
                        }
                        let game_player = GamePlayer{
                            player_index: player_ref.index(),
                            host: false,
                        };
                        players.insert(new_id, game_player);
                        player_ref.connect(game, send.clone());
                        
                        send.send(ToClientPacket::AcceptJoin{in_game: true});
                        
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
                if !players.iter().any(|p|p.1.host) {
                    if let Some(new_host) = players.values_mut().next(){
                        new_host.set_host();
                    }
                }

                settings.role_list.pop();

                Self::send_players_lobby(players);
                for player in players.iter(){
                    Self::send_settings(player.1, settings)
                }
            },
            LobbyState::Game {game, players} => {
                //TODO proper disconnect from game
                let player_index = players.get(&id);
                if let Some(game_player) = player_index {
                    if let Ok(player_ref) = PlayerReference::new(game, game_player.player_index) {
                        if !player_ref.has_left(game) {
                            player_ref.lose_connection(game);
                        }
                    }
                }
                players.remove(&id);
                if !players.iter().any(|p|p.1.host) {
                    if let Some(new_host) = players.values_mut().next(){
                        new_host.host = true;
                    }
                }
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
            
            if PlayerReference::all_players(game).iter().all(|p| p.has_left(game)) {
                self.lobby_state = LobbyState::Closed;
            }
        }
    }
    
    /// Catches the sender up with the current lobby settings
    pub fn send_settings(player: &LobbyPlayer, settings: &Settings) {
        player.send(ToClientPacket::PhaseTimes { phase_time_settings: settings.phase_times.clone() });
        player.send(ToClientPacket::RoleList { role_list: settings.role_list.clone() });
        player.send(ToClientPacket::ExcludedRoles { roles: settings.excluded_roles.clone()});
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
        desired_name = desired_name
            .remove_newline()
            .trim_whitespace()
            .truncate(MAX_NAME_LENGTH)
            .truncate_lines(1);
    
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
                    let new_random_name = new_random_name
                        .remove_newline()
                        .trim_whitespace()
                        .truncate(MAX_NAME_LENGTH)
                        .truncate_lines(1);

                    let existing_name = existing_name.to_string()
                        .remove_newline()
                        .trim_whitespace()
                        .truncate(MAX_NAME_LENGTH)
                        .truncate_lines(1);

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
