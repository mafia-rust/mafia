
pub mod lobby_client;
pub mod game_client;
pub mod on_client_message;
mod name_validation;

use std::time::Duration;

use game_client::GameClientLocation;
use lobby_client::Ready;

use crate::{
    client_connection::ClientConnection, game::{
        player::PlayerReference, role_list::RoleOutline, settings::Settings, spectator::{spectator_pointer::SpectatorPointer, SpectatorInitializeParameters}, Game
    }, packet::{HostDataPacketGameClient, RejectJoinReason, ToClientPacket}, vec_map::VecMap, websocket_connections::connection::ClientSender, websocket_listener::RoomCode
};


use self::{game_client::GameClient, lobby_client::{LobbyClient, LobbyClientID, LobbyClientType}};

pub struct Lobby {
    room_code: RoomCode,
    pub name: String,
    lobby_state: LobbyState,
}

enum LobbyState {
    Lobby {
        settings: Settings,
        clients: VecMap<LobbyClientID, LobbyClient>,
    },
    Game {
        game: Game,
        clients: VecMap<LobbyClientID, GameClient>,
    },
    Closed
}

pub const LOBBY_DISCONNECT_TIMER_SECS: u64 = 5;
pub const GAME_DISCONNECT_TIMER_SECS: u16 = 60 * 2;


impl Lobby {
    pub fn new(room_code: RoomCode) -> Lobby {
        Self { 
            room_code,
            name: name_validation::DEFAULT_SERVER_NAME.to_string(),
            lobby_state: LobbyState::Lobby{
                settings: Settings::default(),
                clients: VecMap::new()
            }
        }
    }

    pub fn is_in_game(&self) -> bool {
        matches!(self.lobby_state, LobbyState::Game { .. })
    }

    pub fn set_rolelist_length(settings: &mut Settings, clients: &VecMap<LobbyClientID, LobbyClient>) {
        let length = clients.iter()
            .filter(|p| matches!(p.1.client_type, LobbyClientType::Player{..}))
            .count();

        settings.role_list.0.resize(length, RoleOutline::default());
    }

    pub fn send_to_client_by_id(&self, lobby_client_id: LobbyClientID, packet: ToClientPacket) {
        match &self.lobby_state {
            LobbyState::Lobby { clients, .. } => {
                if let Some(player) = clients.get(&lobby_client_id) {
                    player.send(packet);
                }
            },
            LobbyState::Game { game, clients, .. } => {
                if let Some(player) = clients.get(&lobby_client_id) {
                    match player.client_location {
                        GameClientLocation::Player(player_index) => {
                            if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                                player_ref.send_packet(game, packet);
                            }
                        },
                        GameClientLocation::Spectator(index) => {
                            SpectatorPointer::new(index).send_packet(game, packet);
                        }
                    }
                }
            },
            LobbyState::Closed => {}
        }
    }

    pub fn get_player_names(clients: &VecMap<LobbyClientID, LobbyClient>) -> Vec<String> {
        clients.values().filter_map(|p| {
            if let LobbyClientType::Player { name } = p.client_type.clone() {
                Some(name)
            } else {
                None
            }
        }).collect()
    }

    pub fn join_player(&mut self, send: &ClientSender) -> Result<LobbyClientID, RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { clients, settings } => {

                let name = name_validation::sanitize_name("".to_string(), &Self::get_player_names(clients));
                
                let new_player = LobbyClient::new(name.clone(), send.clone(), clients.is_empty());
                let Some(lobby_client_id) =
                    (clients
                        .iter()
                        .map(|(i,_)|*i)
                        .fold(0u32, u32::max) as LobbyClientID).checked_add(1) else {
                            send.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomFull });
                            return Err(RejectJoinReason::RoomFull)
                        };

                clients.insert(lobby_client_id, new_player);

                Self::ensure_host_lobby(clients, None);

                Lobby::set_rolelist_length(settings, clients);

                send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: false, player_id: lobby_client_id, spectator: false});

                Self::send_players_lobby(clients);

                for player in clients.iter(){
                    Self::send_settings(player.1, settings, self.name.clone())
                }
                
                send.send(ToClientPacket::LobbyName { name: self.name.clone() });
                Ok(lobby_client_id)
            },
            LobbyState::Game{ clients, game} => {

                let is_host = !clients.iter().any(|p|p.1.host);
                
                let Some(lobby_client_id) = 
                    (clients
                        .iter()
                        .map(|(i,_)|*i)
                        .fold(0u32, u32::max) as LobbyClientID).checked_add(1) else {
                            send.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomFull });
                            return Err(RejectJoinReason::RoomFull);
                        };

                Self::ensure_host_game(game, clients, None);

                let new_spectator = game.join_spectator(SpectatorInitializeParameters {
                    connection: ClientConnection::Connected(send.clone()),
                    host: is_host,
                })?;

                send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: true, player_id: lobby_client_id, spectator: true});
                new_spectator.send_join_game_data(game);
                
                let new_client = GameClient::new_spectator(new_spectator.index(), is_host);

                clients.insert(lobby_client_id, new_client);

                Self::resend_host_data_to_all_hosts(game, clients);

                send.send(ToClientPacket::LobbyName { name: self.name.clone() });
                Ok(lobby_client_id)
            }
            LobbyState::Closed => {
                send.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
                Err(RejectJoinReason::RoomDoesntExist)
            }
        }
    }
    pub fn remove_player(&mut self, lobby_client_id: LobbyClientID) {
        match &mut self.lobby_state {
            LobbyState::Lobby { clients, settings } => {
                let player = clients.remove(&lobby_client_id);
        
                if clients.is_empty() {
                    self.lobby_state = LobbyState::Closed;
                    return;
                }
                Self::ensure_host_lobby(clients, None);

                if let Some(_player) = player {
                    Lobby::set_rolelist_length(settings, clients);
                };

                Self::send_players_lobby(clients);
                for player in clients.iter(){
                    Self::send_settings(player.1, settings, self.name.clone());
                }
            },
            LobbyState::Game { game, clients } => {
                let Some(game_player) = clients.get_mut(&lobby_client_id) else {return};
                match game_player.client_location {
                    GameClientLocation::Player(player_index) => {
                        if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                            player_ref.quit(game);
                        }
                    },
                    GameClientLocation::Spectator(idx) => {
                        clients.remove(&lobby_client_id);
                        for client in clients.iter_mut() {
                            if let GameClientLocation::Spectator(index) = &mut client.1.client_location {
                                if *index > idx {
                                    *index = index.saturating_sub(1);
                                }
                            }
                        }
                        game.remove_spectator(idx);
                    }
                }

                Self::ensure_host_game(game, clients, None);

                Self::resend_host_data_to_all_hosts(game, clients);
            },
            LobbyState::Closed => {}
        }
    }
    pub fn remove_player_rejoinable(&mut self, id: LobbyClientID) {
        match &mut self.lobby_state {
            LobbyState::Lobby {clients, settings: _settings} => {
                let Some(client) = clients.get_mut(&id) else {return};

                if client.is_spectator() {
                    self.remove_player(id);
                    return;
                }

                client.connection = ClientConnection::CouldReconnect { 
                    disconnect_timer: Duration::from_secs(LOBBY_DISCONNECT_TIMER_SECS)
                };

                Self::ensure_host_lobby(clients, None);

                Self::send_players_lobby(clients);
                
            },
            LobbyState::Game {game, clients: players} => {
                let Some(game_player) = players.get_mut(&id) else {return};

                if let GameClientLocation::Player(player_index) = game_player.client_location {
                    if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                        if !player_ref.is_disconnected(game) {
                            player_ref.lose_connection(game);

                            Self::ensure_host_game(game, players, None);
                            Self::resend_host_data_to_all_hosts(game, players);
                        }
                    }
                }
                
            },
            LobbyState::Closed => {}
        }
    }
    pub fn rejoin_player(&mut self, send: &ClientSender, lobby_client_id: LobbyClientID) -> Result<(), RejectJoinReason>{
        match &mut self.lobby_state {
            LobbyState::Lobby { clients: players, settings } => {
                let Some(player) = players.get_mut(&lobby_client_id) else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    return Err(RejectJoinReason::PlayerDoesntExist)
                };
                if let ClientConnection::CouldReconnect { .. } = &mut player.connection {
                    player.connection = ClientConnection::Connected(send.clone());
                    send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: false, player_id: lobby_client_id, spectator: false});

                    Self::send_settings(player, settings, self.name.clone());
                    Self::send_players_lobby(players);
                    
                    send.send(ToClientPacket::LobbyName { name: self.name.clone() });
                    Ok(())
                } else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    Err(RejectJoinReason::PlayerDoesntExist)
                }
            },
            LobbyState::Game { game, clients: players } => {
                let Some(game_player) = players.get_mut(&lobby_client_id) else {
                    send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                    return Err(RejectJoinReason::PlayerDoesntExist)
                };
                
                if let GameClientLocation::Player(player_index) = game_player.client_location {
                    let Ok(player_ref) = PlayerReference::new(game, player_index) else {
                        send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
                        return Err(RejectJoinReason::PlayerDoesntExist)
                    };
                    if !player_ref.could_reconnect(game) {
                        send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerTaken});
                        return Err(RejectJoinReason::PlayerTaken)
                    };
    
                    send.send(ToClientPacket::AcceptJoin{room_code: self.room_code, in_game: true, player_id: lobby_client_id, spectator: false});
                    player_ref.connect(game, send.clone());

                    send.send(ToClientPacket::PlayersHost{hosts:
                        players
                            .iter()
                            .filter(|p|p.1.host)
                            .map(|p|*p.0)
                            .collect()
                    });

                    Self::resend_host_data_to_all_hosts(game, players);
                    
                    send.send(ToClientPacket::LobbyName { name: self.name.clone() });
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

    pub fn set_player_name(lobby_client_id: LobbyClientID, name: String, clients: &mut VecMap<LobbyClientID, LobbyClient>) {
        let mut other_players = clients.clone();
        other_players.remove(&lobby_client_id);
        
        let new_name: String = name_validation::sanitize_name(name, &Self::get_player_names(&other_players));

        if let Some(player) = clients.get_mut(&lobby_client_id){
            if let LobbyClientType::Player { name } = &mut player.client_type {
                *name = new_name;
            }
        }

        Self::send_players_lobby(clients);
    }

    pub fn set_player_name_game(game: &mut Game, player_ref: PlayerReference, name: String) {
        let mut other_players: Vec<String> = PlayerReference::all_players(game)
            .map(|p| p.name(game))
            .cloned()
            .collect();

        other_players.remove(player_ref.index() as usize);
        
        let new_name: String = name_validation::sanitize_name(name, &other_players);

        player_ref.set_name(game, new_name);
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.lobby_state, LobbyState::Closed)
    }

    pub fn tick(&mut self, time_passed: Duration){
        match &mut self.lobby_state {
            LobbyState::Game { game, .. } => {
                game.tick(time_passed);
                
                if !game.is_any_client_connected() {
                    self.lobby_state = LobbyState::Closed;
                }
            }
            LobbyState::Lobby { settings: _settings, clients: players } => {
                let mut to_remove = vec![];

                for player in players.iter_mut() {
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
            LobbyState::Game { game, clients: players } => {
                players.iter()
                    .filter_map(|(id, player)|
                        if let GameClientLocation::Player(player_index) = player.client_location {
                            if let Ok(player_ref) = PlayerReference::new(game, player_index) {
                                Some((*id, player_ref.name(game).clone()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    )
                    .collect()
            },
            LobbyState::Closed => Vec::new(),
        }
    }
    pub fn is_host(&self, lobby_client_id: LobbyClientID)->bool{
        match &self.lobby_state {
            LobbyState::Lobby { clients: players, .. } => {
                if let Some(player) = players.get(&lobby_client_id){
                    player.is_host()
                }else{
                    false
                }
            },
            LobbyState::Game { clients: players, .. } => {
                if let Some(player) = players.get(&lobby_client_id){
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
        client.send(ToClientPacket::EnabledRoles { roles: settings.enabled_roles.clone().into_iter().collect() });
        client.send(ToClientPacket::EnabledModifiers { modifiers: settings.enabled_modifiers.clone().into_iter().collect() });
    }

    //send the list of players to all players while in the lobby
    fn send_players_lobby(clients: &VecMap<LobbyClientID, LobbyClient>){
        let packet = ToClientPacket::LobbyClients { 
            clients: clients.clone()
        };
        for client in clients.iter() {
            client.1.send(packet.clone());
        }

        //send hosts
        let hosts: Vec<LobbyClientID> = clients.iter().filter(|p|p.1.is_host()).map(|p|*p.0).collect();
        let ready: Vec<LobbyClientID> = clients.iter().filter(|p|p.1.ready == Ready::Ready).map(|p|*p.0).collect();
        let host_packet = ToClientPacket::PlayersHost { hosts };
        let ready_packet = ToClientPacket::PlayersReady { ready };
        // Send Players that have lost connection
        let lost_connection: Vec<LobbyClientID> = clients.iter().filter(|p| matches!(p.1.connection, ClientConnection::CouldReconnect { .. })).map(|p|*p.0).collect();
        let lost_connection_packet = ToClientPacket::PlayersLostConnection { lost_connection };
        
        for client in clients.iter() {
            client.1.send(host_packet.clone());
            client.1.send(ready_packet.clone());
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

    fn send_to_all(&self, packet: ToClientPacket){
        match &self.lobby_state {
            LobbyState::Lobby { clients, .. } => {
                for player in clients.iter() {
                    player.1.send(packet.clone());
                }
            }
            LobbyState::Game { game, .. } => {
                game.send_packet_to_all(packet.clone());
            }
            LobbyState::Closed => {}
        }
    }

    fn resend_host_data_to_all_hosts(game: &Game, clients: &VecMap<LobbyClientID, GameClient>) {
        for client in clients.values().filter(|client| client.host) {
            let client_connection = match client.client_location {
                GameClientLocation::Player(index) => PlayerReference::new(game, index).map(|p| p.connection(game).clone()).ok(),
                GameClientLocation::Spectator(index) => Some(SpectatorPointer::new(index).connection(game))
            };

            if let Some(ClientConnection::Connected(host_sender)) = client_connection {
                Self::resend_host_data(game, clients, &host_sender)
            }
        }
    }
    
    fn resend_host_data(game: &Game, clients: &VecMap<LobbyClientID, GameClient>, send: &ClientSender) {
        send.send(ToClientPacket::HostData { clients: clients.iter()
            .map(|(id, client)| {
                (*id, HostDataPacketGameClient {
                    client_type: client.client_location.clone(),
                    connection: match client.client_location {
                        GameClientLocation::Player(index) => {
                            unsafe { PlayerReference::new_unchecked(index) }.connection(game).clone()
                        }
                        GameClientLocation::Spectator(index) => {
                            SpectatorPointer::new(index).connection(game)
                        }
                    },
                    host: client.host
                })
            }).collect()
        });
    }

    fn ensure_host_lobby(clients: &mut VecMap<LobbyClientID, LobbyClient>, skip: Option<LobbyClientID>) {
        if !clients.iter().any(|p|p.1.is_host()) {
            let next_available_player = clients.iter_mut()
                .filter(|(&id, _)| skip.is_none_or(|s| s != id))
                .map(|(_, c)| c).next();

            if let Some(new_host) = next_available_player {
                new_host.set_host();
            } else if let Some(new_host) = clients.values_mut().next(){
                new_host.set_host();
            }
        }
    }

    fn ensure_host_game(game: &mut Game, clients: &mut VecMap<LobbyClientID, GameClient>, skip: Option<LobbyClientID>) {
        fn is_player_not_disconnected(game: &mut Game, p: (&u32, &GameClient)) -> bool {
            match p.1.client_location {
                GameClientLocation::Spectator(index) => {
                    !matches!(SpectatorPointer::new(index).connection(game), ClientConnection::Disconnected)
                },
                GameClientLocation::Player(index) => {
                    PlayerReference::new(game, index).is_ok_and(|player_ref| 
                        !matches!(player_ref.connection(game), ClientConnection::Disconnected)
                    )
                }
            }
        }
        fn is_player_not_disconnected_host(game: &mut Game, p: (&u32, &GameClient)) -> bool {
            p.1.host && is_player_not_disconnected(game, p)
        }
        fn set_player_host(game: &mut Game, new_host: &mut GameClient) {
            new_host.set_host();
            if let GameClientLocation::Spectator(index) = new_host.client_location {
                SpectatorPointer::new(index).set_host(game, true);
            }
        }

        if !clients.iter().any(|p| is_player_not_disconnected_host(game, p)) {
            let next_available_player = clients.iter_mut()
                .filter(|(&id, _)| skip.is_none_or(|s| s != id))
                .filter(|(id, c)| is_player_not_disconnected(game, (*id, c)))
                .map(|(_, c)| c).next();

            if let Some(new_host) = next_available_player {
                set_player_host(game, new_host);
            } else if let Some(new_host) = clients.values_mut().next(){
                set_player_host(game, new_host);
            }
        }
    }
}
