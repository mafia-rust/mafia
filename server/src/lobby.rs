use std::{
    collections::{
        HashMap,
        HashSet,
        VecDeque
    },
    time::{
        Duration,
        Instant
    }
};

use crate::{
    client_connection::ClientConnection,
    game::{
        player::{
            PlayerIndex,
            PlayerInitializeParameters,
            PlayerReference
        },
        phase::PhaseType,
        role_list::RoleOutline,
        settings::Settings,
        spectator::SpectatorInitializeParameters,
        Game
    },
    listener::RoomCode,
    log,
    packet::{
        RejectJoinReason,
        ToClientPacket,
        ToServerPacket
    },
    websocket_connections::connection::ClientSender
};

pub type LobbyClientID = u32;

use self::name_validation::sanitize_server_name;

pub struct Lobby {
    room_code: RoomCode,
    pub name: String,
    lobby_state: LobbyState,
}

enum LobbyState {
    Lobby {
        settings: Settings,
        clients: HashMap<LobbyClientID, LobbyClient>,
    },
    Game {
        game: Game,
        players: HashMap<LobbyClientID, GameClient>,
    },
    Closed
}

pub const LOBBY_DISCONNECT_TIMER_SECS: u64 = 5;
pub const GAME_DISCONNECT_TIMER_SECS: u64 = 60 * 2;
pub const MESSAGE_PER_SECOND_LIMIT: u64 = 2;
pub const MESSAGE_PER_SECOND_LIMIT_TIME: Duration = Duration::from_secs(2);



#[derive(Clone, Debug)]
pub struct LobbyClient{
    pub connection: ClientConnection,
    pub host: bool,
    pub client_type: LobbyClientType,
}
#[derive(Clone, Debug)]
pub enum LobbyClientType{
    Spectator,
    Player{
        name: String,
    }
}

impl LobbyClient {
    pub fn new(name: String, connection: ClientSender, host: bool)->Self{
        LobbyClient{
           connection: ClientConnection::Connected(connection), host, client_type: LobbyClientType::Player{name}
        }
    }
    pub fn set_host(&mut self) {
        self.host = true;
    }

    pub fn send(&self, message: ToClientPacket) {
        if let ClientConnection::Connected(ref sender) = self.connection {
            sender.send(message);
        }
    }
}
#[derive(Clone, Debug)]
pub struct GameClient{
    pub client_location: GameClientLocation,
    pub host: bool,

    pub last_message_times: VecDeque<Instant>,
}
#[derive(Clone, Debug)]
pub enum GameClientLocation {
    Player(PlayerIndex),
    Spectator
}

impl Lobby {
    #[allow(clippy::new_without_default)]
    pub fn new(room_code: RoomCode) -> Lobby {
        Self { 
            room_code,
            name: name_validation::DEFAULT_SERVER_NAME.to_string(),
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                clients: HashMap::new()
            }
        }
    }

    
    pub fn on_client_message(&mut self, send: &ClientSender, player_id: LobbyClientID, incoming_packet: ToServerPacket){

        //RATE LIMITER
        match incoming_packet {
            ToServerPacket::Vote { .. } |
            ToServerPacket::Judgement { .. } |
            ToServerPacket::Target { .. } |
            ToServerPacket::DayTarget { .. } |
            ToServerPacket::SendMessage { .. } |
            ToServerPacket::SendWhisper { .. } => {
                let LobbyState::Game { players, .. } = &mut self.lobby_state else {
                    return;
                };

                let Some(game_player) = players.get_mut(&player_id) else {
                    log!(error "LobbyState::Game"; "{} {:?}", "Message recieved from player not in game", incoming_packet);
                    return;
                };

                let now = Instant::now();
                while let Some(time) = game_player.last_message_times.front() {
                    if now.duration_since(*time) > MESSAGE_PER_SECOND_LIMIT_TIME {
                        game_player.last_message_times.pop_front();
                    } else {
                        break;
                    }
                }
                if game_player.last_message_times.len() >= (MESSAGE_PER_SECOND_LIMIT_TIME.as_secs() * MESSAGE_PER_SECOND_LIMIT) as usize {
                    send.send(ToClientPacket::RateLimitExceeded);
                    return;
                }
                game_player.last_message_times.push_back(now);
                
            },
            _ => {}
        }



        match incoming_packet {
            ToServerPacket::SetSpectator { spectator } => {
                let LobbyState::Lobby { clients, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", player_id);
                    return
                };
                
                let new_name = name_validation::sanitize_name("".to_string(), &clients);
                if let Some(player) = clients.get_mut(&player_id){
                    match &player.client_type {
                        LobbyClientType::Spectator => {
                            if !spectator {
                                player.client_type = LobbyClientType::Player { name: new_name}
                            }
                        },
                        LobbyClientType::Player { .. } => {
                            if spectator {
                                player.client_type = LobbyClientType::Spectator;
                            }
                        },
                    }
                }

                Self::send_players_lobby(clients);
            }
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { clients: players, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", player_id);
                    return
                };

                let mut other_players = players.clone();
                other_players.remove(&player_id);
                
                let new_name: String = name_validation::sanitize_name(name, &other_players);
                if let Some(player) = players.get_mut(&player_id){
                    if let LobbyClientType::Player { name } = &mut player.client_type {
                        *name = new_name;
                    }
                }

                Self::send_players_lobby(players);
            },
            ToServerPacket::SetLobbyName{ name } => {
                let LobbyState::Lobby { .. } = self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetLobbyName can not be used outside of LobbyState::Lobby", player_id);
                    return
                };

                if !self.is_host(player_id) {return};

                let name = sanitize_server_name(name);
                let name = if name.is_empty() {
                    self.name = name_validation::DEFAULT_SERVER_NAME.to_string();
                    self.name.clone()
                } else {
                    self.name = name.clone();
                    self.name.clone()
                };
                
                self.send_to_all(ToClientPacket::LobbyName { name })
            },
            ToServerPacket::StartGame => {
                let LobbyState::Lobby { settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::StartGame can not be used outside of LobbyState::Lobby", player_id);
                    return
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }

                settings.role_list.simplify();
                let role_list = settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });

                let mut player_indices: HashMap<LobbyClientID, GameClient> = HashMap::new();
                let mut game_players = Vec::new();
                let mut game_spectators = Vec::new();


                let LobbyState::Lobby { settings, clients} = &mut self.lobby_state else {
                    unreachable!("LobbyState::Lobby was checked to be to LobbyState::Lobby in the previous line")
                };

                let mut next_player_index: PlayerIndex = 0;
                for (lobby_client_id, lobby_client) in clients.clone().into_iter() {
                    
                    player_indices.insert(lobby_client_id, if let LobbyClientType::Spectator = lobby_client.client_type {
                        GameClient {
                            client_location: GameClientLocation::Spectator,
                            host: lobby_client.host,
                            last_message_times: VecDeque::new(),
                        }
                    } else {
                        GameClient {
                            client_location: GameClientLocation::Player(next_player_index),
                            host: lobby_client.host,
                            last_message_times: VecDeque::new(),
                        }
                    });
                    
                    match lobby_client.client_type {
                        LobbyClientType::Player { name } => {
                            game_players.push(PlayerInitializeParameters{
                                connection: lobby_client.connection,
                                name,
                                host: lobby_client.host,
                            });
                            next_player_index += 1;
                        },
                        LobbyClientType::Spectator => {
                            game_spectators.push(SpectatorInitializeParameters{
                                connection: lobby_client.connection,
                                host: lobby_client.host,
                            });
                        }   
                    }
                }

                let game = match Game::new(settings.clone(), game_players, game_spectators){
                    Ok(game) => game,
                    Err(err) => {
                        send.send(ToClientPacket::RejectStart { reason: err });
                        log!(info "Lobby"; "Failed to start game: {:?}", err);
                        return
                    }
                };
                
                log!(info "Lobby"; "Game started with room code {}", self.room_code);

                self.lobby_state = LobbyState::Game{
                    game,
                    players: player_indices,
                };
                let LobbyState::Game { game, players: _player } = &mut self.lobby_state else {
                    unreachable!("LobbyState::Game was set to be to LobbyState::Game in the previous line");
                };

                Lobby::send_players_game(game);
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                let LobbyState::Lobby{ settings, clients: players  } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }

                match phase {
                    PhaseType::Briefing => { settings.phase_times.briefing = time; }
                    PhaseType::Obituary => { settings.phase_times.obituary = time; }
                    PhaseType::Discussion => { settings.phase_times.discussion = time; }
                    PhaseType::FinalWords => { settings.phase_times.final_words = time; }
                    PhaseType::Dusk => { settings.phase_times.dusk = time; }
                    PhaseType::Judgement => { settings.phase_times.judgement = time; }
                    PhaseType::Night => { settings.phase_times.night = time; }
                    PhaseType::Testimony => { settings.phase_times.testimony = time; }
                    PhaseType::Nomination => { settings.phase_times.nomination = time; }
                };
                
                self.send_to_all(ToClientPacket::PhaseTime { phase, time });
            },
            ToServerPacket::SetPhaseTimes { phase_time_settings } => {
                let LobbyState::Lobby{ settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }

                settings.phase_times = phase_time_settings.clone();

                self.send_to_all(ToClientPacket::PhaseTimes { phase_time_settings });
            }
            ToServerPacket::SetRoleList { mut role_list } => {
                let LobbyState::Lobby{ settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }


                while role_list.0.len() < players.len() {
                    role_list.0.push(RoleOutline::Any);
                }
                while players.len() < role_list.0.len() {
                    role_list.0.pop();
                }
                settings.role_list = role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetRoleOutline { index, role_outline } => {
                let LobbyState::Lobby{ settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }

                if settings.role_list.0.len() <= index as usize {return}
                let Some(unset_outline) = settings.role_list.0.get_mut(index as usize) else {return};
                *unset_outline = role_outline.clone();
                
                self.send_to_all(ToClientPacket::RoleOutline { index, role_outline });
            }
            ToServerPacket::SimplifyRoleList => {
                let LobbyState::Lobby{ settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return}
                }

                settings.role_list.simplify();
                let role_list = settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetExcludedRoles {mut roles } => {
                let LobbyState::Lobby{ settings, clients: players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", player_id);
                    return;
                };
                if let Some(player) = players.get(&player_id){
                    if !player.host {return;}
                }


                let roles = roles.drain(..).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
                settings.excluded_roles = roles.clone();
                self.send_to_all(ToClientPacket::ExcludedRoles { roles });
            }
            ToServerPacket::Leave => {
                self.remove_player(player_id);
            }
            _ => {
                let LobbyState::Game { game, players } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {:?}", "ToServerPacket not implemented for lobby was sent during lobby: ", incoming_packet);
                    return;
                };
                
                if let GameClientLocation::Player(player_index) = players[&player_id].client_location {
                    game.on_client_message(player_index, incoming_packet)
                }
            }
        }
    }

    pub fn join_player(&mut self, send: &ClientSender) -> Result<LobbyClientID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { clients: players, settings } => {

                let name = name_validation::sanitize_name("".to_string(), players);
                
                let mut new_player = LobbyClient::new(name.clone(), send.clone(), players.is_empty());
                let player_id: LobbyClientID = 
                    players
                        .iter()
                        .map(|(i,_)|*i)
                        .fold(0u32, u32::max) as LobbyClientID + 1u32;

                //if there are no hosts, make this player the host
                if !players.iter().any(|p|p.1.host) {
                    new_player.set_host();
                }

                players.insert(player_id, new_player);

                settings.role_list.0.push(RoleOutline::Any);

                send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: false, player_id});

                Self::send_players_lobby(players);

                for player in players.iter(){
                    Self::send_settings(player.1, settings, self.name.clone())
                }
                
                Ok(player_id)
            },
            LobbyState::Game{ .. } => {
                send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::GameAlreadyStarted});
                Err(RejectJoinReason::GameAlreadyStarted)
            }
            LobbyState::Closed => {
                send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::RoomDoesntExist});
                Err(RejectJoinReason::RoomDoesntExist)
            }
        }
    }
    pub fn remove_player(&mut self, player_id: LobbyClientID) {
        match &mut self.lobby_state {
            LobbyState::Lobby { clients: players, settings } => {
                let player = players.remove(&player_id);
        
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
                    settings.role_list.0.pop();
                };

                Self::send_players_lobby(players);
                for player in players.iter(){
                    Self::send_settings(player.1, settings, self.name.clone());
                }
            },
            LobbyState::Game { game, players } => {
                let Some(game_player) = players.get_mut(&player_id) else {return};
                if let GameClientLocation::Player(player_index) = game_player.client_location {
                    if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                        player_ref.quit(game);
                    }
                }
            },
            LobbyState::Closed => {}
        }
    }
    pub fn remove_player_rejoinable(&mut self, id: LobbyClientID) {

        match &mut self.lobby_state {
            LobbyState::Lobby {clients: players, settings: _settings} => {
                let Some(player) = players.get_mut(&id) else {return};

                player.connection = ClientConnection::CouldReconnect { 
                    disconnect_timer: Duration::from_secs(LOBBY_DISCONNECT_TIMER_SECS)
                };
                Self::send_players_lobby(players);
                
            },
            LobbyState::Game {game, players} => {
                let Some(game_player) = players.get_mut(&id) else {return};

                if let GameClientLocation::Player(player_index) = game_player.client_location {
                    if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                        if !player_ref.is_disconnected(game) {
                            player_ref.lose_connection(game);
                        }
                    }
                }
                
            },
            LobbyState::Closed => {}
        }
    }
    pub fn rejoin_player(&mut self, send: &ClientSender, player_id: LobbyClientID) -> Result<(), RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { clients: players, settings } => {
                let Some(player) = players.get_mut(&player_id) else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    return Err(RejectJoinReason::PlayerDoesntExist)
                };
                if let ClientConnection::CouldReconnect { .. } = &mut player.connection {
                    player.connection = ClientConnection::Connected(send.clone());
                    send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: false, player_id});

                    Self::send_settings(player, settings, self.name.clone());
                    Self::send_players_lobby(players);
                    
                    Ok(())
                } else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    Err(RejectJoinReason::PlayerDoesntExist)
                }
            },
            LobbyState::Game { game, players } => {
                let Some(game_player) = players.get_mut(&player_id) else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    return Err(RejectJoinReason::PlayerDoesntExist)
                };
                
                if let GameClientLocation::Player(player_index) = game_player.client_location {
                    let Ok(player_ref) = PlayerReference::new(game, player_index) else {
                        unreachable!()
                    };
                    if !player_ref.could_reconnect(game) {
                        send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerTaken});
                        return Err(RejectJoinReason::PlayerTaken)
                    };
    
                    send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: true, player_id});
                    player_ref.connect(game, send.clone());
                    
                    Ok(())
                }else{
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    Err(RejectJoinReason::PlayerDoesntExist)
                }
                

            },
            LobbyState::Closed => {
                send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::RoomDoesntExist});
                Err(RejectJoinReason::RoomDoesntExist)
            },
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.lobby_state, LobbyState::Closed)
    }

    pub fn tick(&mut self, time_passed: Duration){
        match &mut self.lobby_state {
            LobbyState::Game { game, .. } => {
                game.tick(time_passed);
                
                if !PlayerReference::all_players(game).any(|p| p.is_connected(game)) {
                    self.lobby_state = LobbyState::Closed;
                }
            }
            LobbyState::Lobby { settings: _settings, clients: players } => {
                let mut to_remove = vec![];

                for player in players {
                    if let ClientConnection::CouldReconnect { disconnect_timer } = &mut player.1.connection {
                        if let Some(time_remaining) = disconnect_timer.checked_sub(time_passed) {
                            *disconnect_timer = time_remaining;
                        } else {
                            player.1.connection = ClientConnection::Disconnected;
                        }
                    }
                    if let ClientConnection::Disconnected = player.1.connection {
                        to_remove.push(*player.0);
                    }
                }

                for player in to_remove {
                    self.remove_player(player);
                }
            },
            LobbyState::Closed => {}
        }
    }

    pub fn get_player_list(&self)->Vec<(LobbyClientID, String)>{
        match &self.lobby_state {
            LobbyState::Lobby { settings:_, clients: players } => {
                players.iter().filter_map(|p|
                    if let LobbyClientType::Player { name } = &p.1.client_type {
                        Some((*p.0, name.clone()))
                    }else{
                        None
                    }
                ).collect()
            },
            LobbyState::Game { game, players } => {
                players.iter()
                    .filter(|(_, player)| matches!(player.client_location, GameClientLocation::Player(_)))
                    .map(|(id, player)| {
                        if let GameClientLocation::Player(player_index) = player.client_location {
                            let player_ref = PlayerReference::new(game, player_index).unwrap();
                            (*id, player_ref.name(game).clone())
                        }else{
                            unreachable!()
                        }
                    })
                    .collect()
            },
            LobbyState::Closed => Vec::new(),
        }
    }
    pub fn is_host(&self, player_id: LobbyClientID)->bool{
        match &self.lobby_state {
            LobbyState::Lobby { clients: players, .. } => {
                if let Some(player) = players.get(&player_id){
                    player.host
                }else{
                    false
                }
            },
            LobbyState::Game { players, .. } => {
                if let Some(player) = players.get(&player_id){
                    player.host
                }else{
                    false
                }
            },
            LobbyState::Closed => false,
        }
    }

    /// Catches the sender up with the current lobby settings
    pub fn send_settings(client: &LobbyClient, settings: &Settings, name: String) {
        client.send(ToClientPacket::LobbyName { name });
        client.send(ToClientPacket::PhaseTimes { phase_time_settings: settings.phase_times.clone() });
        client.send(ToClientPacket::RoleList { role_list: settings.role_list.clone() });
        client.send(ToClientPacket::ExcludedRoles { roles: settings.excluded_roles.clone()});
    }

    //send the list of players to all players while in the lobby
    fn send_players_lobby(clients: &HashMap<LobbyClientID, LobbyClient>){
        let packet = ToClientPacket::LobbyPlayers { 
            players: clients.iter().filter_map(|p|
                if let LobbyClientType::Player { name } = &p.1.client_type {
                    Some((*p.0, name.clone()))
                }else{
                    None
                }
            ).collect()
        };
        for client in clients.iter() {
            client.1.send(packet.clone());
        }

        //send hosts
        let hosts: Vec<LobbyClientID> = clients.iter().filter(|p|p.1.host).map(|p|*p.0).collect();
        let host_packet = ToClientPacket::PlayersHost { hosts };
        // Send Players that have lost connection
        let lost_connection: Vec<LobbyClientID> = clients.iter().filter(|p| matches!(p.1.connection, ClientConnection::CouldReconnect { .. })).map(|p|*p.0).collect();
        let lost_connection_packet = ToClientPacket::PlayersLostConnection { lost_connection };
        
        for client in clients.iter() {
            client.1.send(host_packet.clone());
            client.1.send(lost_connection_packet.clone());
        }
    }
    
    fn send_players_game(game: &mut Game){

        let players: Vec<String> = PlayerReference::all_players(game).map(|p|
            p.name(game).clone()
        ).collect();

        let packet = ToClientPacket::GamePlayers{ 
            players
        };

        game.send_packet_to_all(packet.clone());
    }

    fn send_to_all(&mut self, packet: ToClientPacket){
        match &mut self.lobby_state {
            LobbyState::Lobby { clients: players, .. } => {
                for player in players.iter() {
                    player.1.send(packet.clone());
                }
            }
            LobbyState::Game { game, .. } => {
                game.send_packet_to_all(packet.clone());
            }
            LobbyState::Closed => {}
        }
    }
}

mod name_validation {
    use std::collections::HashMap;
    use crate::{lobby::LobbyClientID, strings::TidyableString};
    use super::{LobbyClient, LobbyClientType};
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
    const MAX_SERVER_NAME_LENGTH: usize = 20;
    pub const DEFAULT_SERVER_NAME: &str = "Mafia Lobby";

    /// Sanitizes a player name.
    /// If the desired name is invalid or taken, this generates a random acceptable name.
    /// Otherwise, this trims and returns the input name.
    pub fn sanitize_name(mut desired_name: String, players: &HashMap<LobbyClientID, LobbyClient>) -> String {
        desired_name = desired_name
            .remove_newline()
            .trim_whitespace()
            .truncate(MAX_NAME_LENGTH)
            .truncate_lines(1);
    
        let name_already_taken = players.values().any(|existing_player|
            if let LobbyClientType::Player { name } = &existing_player.client_type {
                desired_name == *name
            }else{
                false
            }
        );
        
        if !desired_name.is_empty() && !name_already_taken {
            desired_name
        } else {
            generate_random_name(&players.values()
                .filter_map(|p|
                    if let LobbyClientType::Player { name } = &p.client_type {
                        Some(name.as_str())
                    }else{
                        None
                    }
                )
                .collect::<Vec<&str>>())
        }
    }

    pub fn sanitize_server_name(desired_name: String) -> String {
        desired_name
            .remove_newline()
            .trim_whitespace()
            .truncate(MAX_SERVER_NAME_LENGTH)
            .truncate_lines(1)
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
            (taken_names.len()).to_string()
        }
    }
}
