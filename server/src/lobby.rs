use std::{collections::{HashMap, HashSet}, time::Duration};

use crate::{
    game::{
        Game, 
        player::{PlayerIndex, PlayerReference}, 
        settings::Settings, 
        role_list::RoleOutline, 
        phase::PhaseType
    },
    listener::{PlayerID, RoomCode}, packet::{ToClientPacket, RejectJoinReason, ToServerPacket}, websocket_connections::connection::ClientSender, log
};

pub struct Lobby {
    room_code: RoomCode,
    lobby_state: LobbyState,
}

enum LobbyState {
    Lobby {
        settings: Settings,
        players: HashMap<PlayerID, LobbyPlayer>,
    },
    Game {
        game: Game,
        players: HashMap<PlayerID, GamePlayer>,
    },
    Closed
}

#[derive(Clone)]
pub struct LobbyPlayer{
    pub sender: ClientSender,
    pub name: String,
    pub host: bool,
    pub to_be_kicked: bool,
}

impl LobbyPlayer {
    pub fn new(name: String, sender: ClientSender, host: bool)->Self{
        LobbyPlayer{
            name, sender, host, to_be_kicked: false
        }
    }
    pub fn set_host(&mut self) {
        self.host = true;
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
    pub fn new(room_code: RoomCode) -> Lobby {
        Self { 
            room_code,
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                players: HashMap::new()
            }
        }
    }

    
    pub fn on_client_message(&mut self, send: &ClientSender, player_id: PlayerID, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { players, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", player_id);
                    return;
                };

                let mut other_players = players.clone();
                other_players.remove(&player_id);
                
                let name = name_validation::sanitize_name(name, &other_players);
                if let Some(player) = players.get_mut(&player_id){
                    player.name = name.clone();
                }

                Self::send_players_lobby(players);
            },
            ToServerPacket::StartGame => {
                let LobbyState::Lobby { settings: _, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::StartGame can not be used outside of LobbyState::Lobby", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }

                let mut player_indices: HashMap<PlayerID,GamePlayer> = HashMap::new();
                let mut game_players = Vec::new();

                

                let LobbyState::Lobby { settings, players} = &mut self.lobby_state else {
                    unreachable!("LobbyState::Lobby was checked to be to LobbyState::Lobby in the previous line")
                };

                for (index, (arbitrary_player_id, lobby_player)) in players.drain().enumerate() {
                    player_indices.insert(arbitrary_player_id, GamePlayer { player_index: index as PlayerIndex, host: lobby_player.host });
                    game_players.push(lobby_player);
                }

                let game = match Game::new(settings.clone(), game_players){
                    Ok(game) => game,
                    Err(err) => {
                        send.send(ToClientPacket::RejectStart { reason: err });
                        log!(info "Lobby"; "Failed to start game: {:?}", err);
                        return;
                    }
                };
                
                self.send_to_all(ToClientPacket::StartGame);

                self.lobby_state = LobbyState::Game{
                    game,
                    players: player_indices,
                };
                let LobbyState::Game { game, players: _player } = &mut self.lobby_state else {
                    unreachable!("LobbyState::Game was set to be to LobbyState::Game in the previous line");
                };

                Lobby::send_players_game(game);
            },
            ToServerPacket::KickPlayer{player_id: kicked_player_id} => {
                let LobbyState::Lobby { players, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::KickPlayer can not be used outside of LobbyState::Lobby", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }

                if let Some(kicked_player) = players.get_mut(&kicked_player_id){
                    kicked_player.to_be_kicked = true;
                }
                self.send_to_all(ToClientPacket::KickPlayer { player_id: kicked_player_id });
                
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                let LobbyState::Lobby{ settings, players  } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
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
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }

                settings.phase_times = phase_time_settings.clone();

                self.send_to_all(ToClientPacket::PhaseTimes { phase_time_settings });
            }
            ToServerPacket::SetRoleList { mut role_list } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }


                while role_list.len() < players.len() {
                    role_list.push(RoleOutline::Any);
                }
                while players.len() < role_list.len() {
                    role_list.pop();
                }
                settings.role_list = role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetRoleOutline { index, role_outline } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }

                settings.role_list[index as usize] = role_outline.clone();
                
                self.send_to_all(ToClientPacket::RoleOutline { index, role_outline });
            }
            ToServerPacket::SetExcludedRoles {mut roles } => {
                let LobbyState::Lobby{ settings, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }


                let roles: HashSet<_> = roles.drain(..).collect();
                let roles: Vec<_> = roles.into_iter().filter(|e|*e!=RoleOutline::Any).map(|e|e.clone()).collect();
                settings.excluded_roles = roles.clone();
                self.send_to_all(ToClientPacket::ExcludedRoles { roles });
            }
            _ => {
                let LobbyState::Game { game, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {:?}", "ToServerPacket not implemented for lobby was sent during lobby: ", incoming_packet);
                    return;
                };

                game.on_client_message(players[&player_id].player_index, incoming_packet)
            }
        }
    }

    pub fn connect_player_to_lobby(&mut self, send: &ClientSender)-> Result<PlayerID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { players, settings } => {
                let name = name_validation::sanitize_name("".to_string(), players);
                
                
                let mut new_player = LobbyPlayer::new(name.clone(), send.clone(), players.is_empty());
                let arbitrary_player_id: PlayerID = 
                    players
                        .iter()
                        .map(|(i,_)|*i)
                        .fold(0u32, u32::max) as PlayerID + 1u32;

                //if there are no hosts, make this player the host
                if !players.iter().any(|p|p.1.host) {
                    new_player.set_host();
                }

                players.insert(arbitrary_player_id, new_player);

                settings.role_list.push(RoleOutline::Any);

                

                send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: false, player_id: arbitrary_player_id});

                Self::send_players_lobby(players);

                for player in players.iter(){
                    Self::send_settings(player.1, settings)
                }
                
                Ok(arbitrary_player_id)
            },
            LobbyState::Game{ game, players } => {

                for player_ref in PlayerReference::all_players(game){
                    if player_ref.has_lost_connection(game) {
                        let arbitrary_player_ids: Vec<PlayerID> = players.keys().copied().collect();
                        let mut new_id: PlayerID = 0;
                        for id in arbitrary_player_ids {
                            if id >= new_id {
                                new_id = id + 1;
                            }
                        }

                        send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: true, player_id: new_id});

                        let game_player = GamePlayer{
                            player_index: player_ref.index(),
                            host: false,
                        };
                        players.insert(new_id, game_player);
                        player_ref.connect(game, send.clone());
                        
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
    pub fn disconnect_player_from_lobby(&mut self, id: PlayerID) {

        //TODO!!! if player id doesnt exist do nothing!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!


        match &mut self.lobby_state {
            LobbyState::Lobby {players, settings} => {
                let player = players.remove(&id);
            
                if players.is_empty() {
                    self.lobby_state = LobbyState::Closed;
                    return;
                }
                if !players.iter().any(|p|p.1.host) {
                    if let Some(new_host) = players.values_mut().next(){
                        new_host.set_host();
                    }
                }

                if let Some(_player) = player {
                    settings.role_list.pop();
                };

                Self::send_players_lobby(players);
                for player in players.iter(){
                    Self::send_settings(player.1, settings);
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
    
    pub fn get_players_to_kick(&self)->Vec<PlayerID>{
        let LobbyState::Lobby { players, .. } = &self.lobby_state else {
            return vec![];
        };
        players.iter().filter(|p|p.1.to_be_kicked).map(|p|*p.0).collect()
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


    fn send_players_lobby(players: &HashMap<PlayerID, LobbyPlayer>){
        let packet = ToClientPacket::LobbyPlayers { 
            players: players.iter().map(|p| {
                (*p.0, p.1.name.clone())
            }).collect()
        };
        for player in players.iter() {
            player.1.send(packet.clone());
        }

        //send hosts
        let hosts: Vec<PlayerID> = players.iter().filter(|p|p.1.host).map(|p|*p.0).collect();
        let packet = ToClientPacket::PlayersHost { hosts };
        for player in players.iter() {
            player.1.send(packet.clone());
        }
    }
    
    fn send_players_game(game: &mut Game){

        let players: Vec<String> = PlayerReference::all_players(game).into_iter().map(|p|
            p.name(game).clone()
        ).collect();

        let packet = ToClientPacket::GamePlayers{ 
            players
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
    use crate::{listener::PlayerID, strings::TidyableString};
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
    pub fn sanitize_name(mut desired_name: String, players: &HashMap<PlayerID, LobbyPlayer>) -> String {
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
            // TODO this can cause a crash if someone is already named this
            (taken_names.len()).to_string()
        }
    }
}
