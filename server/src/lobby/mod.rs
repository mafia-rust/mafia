pub mod lobby_client;
pub mod on_client_message;

use std::{collections::VecDeque, time::{Duration, Instant}};

use lobby_client::{LobbyClient, LobbyClientType, Ready};

use crate::{client_connection::ClientConnection, game::{role_list::RoleOutline, settings::Settings}, packet::{RoomPreviewData, RejectJoinReason, ToClientPacket}, room::{name_validation, JoinClientData, RemoveClientData, RoomClientID, RoomState, TickData}, vec_map::VecMap, websocket_connections::connection::ClientSender};

pub struct Lobby {
    pub name: String,
    pub settings: Settings,
    pub clients: VecMap<RoomClientID, LobbyClient>,
}

impl Lobby {
    const DISCONNECT_TIMER_SECS: u64 = 5;

    pub fn new() -> Self {
        Self {
            name: name_validation::DEFAULT_SERVER_NAME.to_string(),
            settings: Settings::default(),
            clients: VecMap::new(),
        }
    }

    pub fn ensure_host_exists(&mut self, skip: Option<RoomClientID>) {
        if !self.clients.iter().any(|p|p.1.is_host()) {
            let next_available_player = self.clients.iter_mut()
                .filter(|(&id, _)| skip.is_none_or(|s| s != id))
                .map(|(_, c)| c).next();

            if let Some(new_host) = next_available_player {
                new_host.set_host();
            } else if let Some(new_host) = self.clients.values_mut().next(){
                new_host.set_host();
            }
        }
    }

    //send the list of players to all players
    pub fn send_players(&self){
        let packet = ToClientPacket::LobbyClients { 
            clients: self.clients.clone()
        };
        for client in self.clients.iter() {
            client.1.send(packet.clone());
        }

        //send hosts
        let hosts: Vec<RoomClientID> = self.clients.iter().filter(|p|p.1.is_host()).map(|p|*p.0).collect();
        let ready: Vec<RoomClientID> = self.clients.iter().filter(|p|p.1.ready == Ready::Ready).map(|p|*p.0).collect();
        let host_packet = ToClientPacket::PlayersHost { hosts };
        let ready_packet = ToClientPacket::PlayersReady { ready };

        // Send Players that have lost connection
        let lost_connection: Vec<RoomClientID> = self.clients.iter().filter(|p| matches!(p.1.connection, ClientConnection::CouldReconnect { .. })).map(|p|*p.0).collect();
        let lost_connection_packet = ToClientPacket::PlayersLostConnection { lost_connection };
        
        for client in self.clients.iter() {
            client.1.send(host_packet.clone());
            client.1.send(ready_packet.clone());
            client.1.send(lost_connection_packet.clone());
        }
    }

    /// Catches the sender up with the current lobby settings
    fn send_settings(&self, send: &ClientSender) {
        send.send(ToClientPacket::RoomName { name: self.name.clone() });
        send.send(ToClientPacket::PhaseTimes { phase_time_settings: self.settings.phase_times.clone() });
        send.send(ToClientPacket::RoleList { role_list: self.settings.role_list.clone() });
        send.send(ToClientPacket::EnabledRoles { roles: self.settings.enabled_roles.clone().into_iter().collect() });
        send.send(ToClientPacket::EnabledModifiers { modifiers: self.settings.enabled_modifiers.clone().into_iter().collect() });
    }

    pub fn set_player_name(&mut self, room_client_id: RoomClientID, name: String) {
        let mut other_players = self.clients.clone();
        other_players.remove(&room_client_id);

        let other_player_names = {
            other_players.values().filter_map(|p| {
                if let LobbyClientType::Player { name } = p.client_type.clone() {
                    Some(name)
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        };
        
        let new_name: String = name_validation::sanitize_name(name, &other_player_names);

        if let Some(player) = self.clients.get_mut(&room_client_id){
            if let LobbyClientType::Player { name } = &mut player.client_type {
                *name = new_name;
            }
        }

        self.send_players();
    }

    pub fn set_rolelist_length(&mut self) {
        let length = self.clients.iter()
            .filter(|p| matches!(p.1.client_type, LobbyClientType::Player{..}))
            .count();

        self.settings.role_list.0.resize(length, RoleOutline::default());
    }
    
    pub fn send_to_all(&self, packet: ToClientPacket) {
        for client in self.clients.iter() {
            client.1.send(packet.clone());
        }
    }
    
    pub fn get_client_last_message_times(&mut self, room_client_id: u32) -> Option<&mut VecDeque<Instant>> {
        if let Some(client) = self.clients.get_mut(&room_client_id) {
            Some(&mut client.last_message_times)
        } else {
            None
        }
    }
    
    pub fn new_from_game(name: String, settings: Settings, clients: VecMap<RoomClientID, LobbyClient>) -> Self {
        let new = Self { name, settings, clients };

        for (id, client) in new.clients.iter() {
            client.send(ToClientPacket::YourId { player_id: *id });
            if let ClientConnection::Connected(send) = &client.connection {
                new.send_settings(send);
            }
        }

        new.send_players();

        new
    }
}

impl RoomState for Lobby {
    fn send_to_client_by_id(&self, room_client_id: RoomClientID, packet: ToClientPacket) {
        if let Some(player) = self.clients.get(&room_client_id) {
            player.send(packet);
        }
    }

    fn join_client(&mut self, send: &ClientSender) -> Result<JoinClientData, RejectJoinReason> {
        let player_names = self.clients.values().filter_map(|p| {
            if let LobbyClientType::Player { name } = p.client_type.clone() {
                Some(name)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        let name = name_validation::sanitize_name("".to_string(), &player_names);
        
        let new_player = LobbyClient::new(name.clone(), send.clone(), self.clients.is_empty());
        let Some(room_client_id) =
            (self.clients
                .iter()
                .map(|(i,_)|*i)
                .fold(0u32, u32::max) as RoomClientID).checked_add(1) else {
                    send.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomFull });
                    return Err(RejectJoinReason::RoomFull)
                };

        self.clients.insert(room_client_id, new_player);

        self.ensure_host_exists(None);

        self.set_rolelist_length();
        for player in self.clients.iter(){
            if let ClientConnection::Connected(send) = &player.1.connection {
                self.send_settings(send);
            }
        }

        Ok(JoinClientData {
            id: room_client_id,
            in_game: false,
            spectator: false
        })
    }
    
    fn remove_client(&mut self, room_client_id: u32) -> RemoveClientData {
        let player = self.clients.remove(&room_client_id);
        
        if self.clients.is_empty() {
            return RemoveClientData {
                close_room: true,
            };
        }

        self.ensure_host_exists(None);

        if let Some(_player) = player {
            self.set_rolelist_length();
        };

        self.send_players();
        for player in self.clients.iter(){
            if let ClientConnection::Connected(send) = &player.1.connection {
                self.send_settings(send);
            }
        }

        RemoveClientData { close_room: false }
    }
    
    fn remove_client_rejoinable(&mut self, id: RoomClientID) -> RemoveClientData {
        let Some(client) = self.clients.get_mut(&id) else {return RemoveClientData { close_room: false }};

        if client.is_spectator() {
            return self.remove_client(id);
        }

        client.connection = ClientConnection::CouldReconnect { 
            disconnect_timer: Duration::from_secs(Self::DISCONNECT_TIMER_SECS)
        };

        self.ensure_host_exists(None);

        self.send_players();

        RemoveClientData { close_room: false }
    }
    
    fn rejoin_client(&mut self, send: &ClientSender, id: RoomClientID) -> Result<JoinClientData, RejectJoinReason> {
        let Some(client) = self.clients.get_mut(&id) else {
            send.send(ToClientPacket::RejectJoin{reason: RejectJoinReason::PlayerDoesntExist});
            return Err(RejectJoinReason::PlayerDoesntExist)
        };
        match &mut client.connection {
            ClientConnection::Connected(_) => Err(RejectJoinReason::PlayerTaken),
            ClientConnection::CouldReconnect { .. } => {
                client.connection = ClientConnection::Connected(send.clone());
    
                Ok(JoinClientData {
                    id,
                    in_game: false,
                    spectator: false
                })
            },
            ClientConnection::Disconnected => Err(RejectJoinReason::PlayerDoesntExist)
        }
    }

    fn initialize_client(&mut self, _room_client_id: RoomClientID, send: &ClientSender) {
        self.send_players();
        self.send_settings(send);
        
        send.send(ToClientPacket::RoomName { name: self.name.clone() });
    }
    
    fn tick(&mut self, time_passed: Duration) -> TickData {
        let mut to_remove = vec![];

        for client in self.clients.iter_mut() {
            if let ClientConnection::CouldReconnect { disconnect_timer } = &mut client.1.connection {
                if let Some(time_remaining) = disconnect_timer.checked_sub(time_passed) {
                    *disconnect_timer = time_remaining;
                } else {
                    client.1.connection = ClientConnection::Disconnected;
                }
            }
            if let ClientConnection::Disconnected = client.1.connection {
                to_remove.push(*client.0);
            }
        }

        for client in to_remove {
            let RemoveClientData { close_room } = self.remove_client(client);
            if close_room {
                return TickData { close_room: true };
            }
        }

        TickData { close_room: false }
    }
    
    fn get_preview_data(&self) -> RoomPreviewData {
        RoomPreviewData { 
            name: self.name.clone(),
            in_game: false,
            players: self.clients.iter().filter_map(|p|
                if let LobbyClientType::Player { name } = &p.1.client_type {
                    Some((*p.0, name.clone()))
                }else{
                    None
                }
            ).collect()
        }
    }
    
    fn is_host(&self, room_client_id: u32) -> bool {
        if let Some(client) = self.clients.get(&room_client_id){
            client.is_host()
        }else{
            false
        }
    }
}