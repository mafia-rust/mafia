use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::Duration};

use rand::random;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    lobby::{lobby_client::LobbyClientID, Lobby}, 
    log, 
    packet::{LobbyPreviewData, RejectJoinReason, ToClientPacket, ToServerPacket}, 
    websocket_connections::connection::Connection
};

pub type RoomCode = usize;

struct ListenerClient {
    connection: Connection,
    location: ListenerClientLocation,
}
impl ListenerClient{
    fn new(connection: Connection) -> Self {
        Self {
            connection,
            location: ListenerClientLocation::OutsideLobby,
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
enum ListenerClientLocation {
    InLobby{
        room_code: RoomCode,
        lobby_client_id: LobbyClientID,
    },
    OutsideLobby
}

pub struct Listener {
    lobbies: HashMap<RoomCode, Lobby>,
    clients: HashMap<SocketAddr, ListenerClient>,
}
impl Listener{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub fn start(listener: Arc<Mutex<Self>>) {
        const DESIRED_FRAME_TIME: Duration = Duration::from_millis(1000);

        tokio::spawn(async move {
            let mut frame_start_time = tokio::time::Instant::now();
            loop {
                let delta_time = frame_start_time.elapsed();
                frame_start_time = tokio::time::Instant::now();

                if let Ok(mut listener) = listener.lock() {
                    let mut closed_lobbies = Vec::new();
                    
                    let Listener { ref mut lobbies, clients: ref _players} = *listener;

                    // log!(info "Listener"; "lobbies: {:?} players: {:?}", lobbies.keys(), _players.len());

                    for (room_code, lobby) in lobbies.iter_mut() {
                        if lobby.is_closed() {
                            closed_lobbies.push(*room_code);
                        } else {
                            lobby.tick(delta_time);
                        }
                    }

                    for key in closed_lobbies {
                        log!(important "Lobby"; "Closed {key}");
                        listener.delete_lobby(key);
                    }
                } else { 
                    return;
                }

                tokio::time::sleep(DESIRED_FRAME_TIME.saturating_sub(tokio::time::Instant::now() - frame_start_time)).await;
            }
        });
    }

    fn create_lobby(&mut self) -> Option<RoomCode>{
        let Some(room_code) = ((random::<u16>() as usize)..usize::MAX).find(
            |code| !self.lobbies.contains_key(code)
        ) else {
            return None;
        };

        let lobby = Lobby::new(room_code);
        self.lobbies.insert(room_code, lobby);
        Some(room_code)
    }
    fn delete_lobby(&mut self, room_code: RoomCode){
        let players_to_remove: Vec<_> = self.clients.iter().filter(|p| 
            if let ListenerClientLocation::InLobby{room_code: player_room_code, ..} = p.1.location {
                player_room_code == room_code 
            }else{
                false
            }
        ).map(|f|*f.0).collect();

        for player in players_to_remove{
            self.set_player_outside_lobby(&player, false);
        }
        self.lobbies.remove(&room_code);
    }

    fn set_player_in_lobby_initial_connect(&mut self, connection: &Connection, room_code: RoomCode){
        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = self.clients
            .get_mut(connection.get_address())
            .map(|p|&mut p.location)
        else{
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        if let Ok(lobby_client_id) = lobby.join_player(&connection.get_sender()) {
            *sender_player_location = ListenerClientLocation::InLobby { room_code, lobby_client_id };
        }
        
        connection.send(ToClientPacket::LobbyName { name: lobby.name.clone() })
    }
    fn set_player_in_lobby_reconnect(&mut self, connection: &Connection, room_code: RoomCode, lobby_client_id: LobbyClientID){

        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = 
            self.clients
            .get_mut(connection.get_address())
            .map(|p|&mut p.location)
        else{
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        if lobby.rejoin_player(&connection.get_sender(), lobby_client_id).is_ok() {
            *sender_player_location = ListenerClientLocation::InLobby { room_code, lobby_client_id };
        }
        
        connection.send(ToClientPacket::LobbyName { name: lobby.name.clone() })
    }
    //returns if player was in the lobby
    fn set_player_outside_lobby(&mut self, address: &SocketAddr, rejoinable: bool) -> bool {
        let Some(sender_player_location) = self.clients
            .get_mut(address)
            .map(|p|&mut p.location)
        else{
            log!(error "Listener"; "{} {}", "Attempted leave a non player that isn't in the map", address);
            return false;
        };
        let ListenerClientLocation::InLobby { room_code, lobby_client_id } = sender_player_location else {return false};
        if let Some(lobby) = self.lobbies.get_mut(room_code) {
            if rejoinable {
                lobby.remove_player_rejoinable(*lobby_client_id);
            }else{
                lobby.remove_player(*lobby_client_id)
            }
        }
        *sender_player_location = ListenerClientLocation::OutsideLobby;
        true
    }
    
    pub fn create_player(&mut self, connection: &Connection) {
        self.clients.insert(*connection.get_address(), ListenerClient::new(connection.clone()));
    }

    pub fn delete_player(&mut self, address: &SocketAddr) -> Result<(), &'static str> {
        let Some(disconnected_player_location) = self.clients
            .remove(address)
            .map(|p|p.location)
        else{
            return Err("Player doesn't exist");
        };

        if let ListenerClientLocation::InLobby { room_code, lobby_client_id } = disconnected_player_location {
            if let Some(lobby) = self.lobbies.get_mut(&room_code) {
                lobby.remove_player(lobby_client_id)
            }
        }

        Ok(())
    }

    fn get_address_from_location(&self, location: ListenerClientLocation) -> Option<SocketAddr> {
        for (address, player) in self.clients.iter() {
            if location == player.location{
                return Some(*address);
            }
        }
        None
    }

    pub fn on_connect(&mut self, connection: &Connection) {
        self.create_player(connection);
    }

    pub fn on_disconnect(&mut self, connection: Connection) -> Result<(), &'static str> {
        self.set_player_outside_lobby(connection.get_address(), true);
        Ok(())
        // self.delete_player(connection.get_address())
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() { return }

        log!(info "Listener"; "{}: {}", &connection.get_address().to_string(), message);
        if let Err(k) = self.handle_message(connection, message){
            log!(error "Listener"; "Serde error when receiving message from {}: {}\n{}", &connection.get_address().to_string(), k, message);
        }
    }

    fn handle_message(&mut self, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {
        let incoming_packet = serde_json::from_str::<ToServerPacket>(message.to_string().as_str())?;

        match incoming_packet {
            ToServerPacket::Ping => {
                connection.send(ToClientPacket::Pong);
            },
            ToServerPacket::LobbyListRequest => {
                connection.send(ToClientPacket::LobbyList{lobbies: self.lobbies.iter()
                    .map(|(room_code, lobby)|
                        (*room_code, LobbyPreviewData { 
                            name: lobby.name.clone(),
                            in_game: lobby.is_in_game(),
                            players: lobby.get_player_list() 
                        }
                    ))
                    .collect::<HashMap<RoomCode, LobbyPreviewData>>()});
            },
            ToServerPacket::ReJoin {room_code, player_id } => {
                self.set_player_in_lobby_reconnect(connection, room_code, player_id);
            }
            ToServerPacket::Join{ room_code } => {
                self.set_player_in_lobby_initial_connect(connection, room_code);
            },
            ToServerPacket::Host => {
                let Some(room_code) = self.create_lobby() else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };
                
                self.set_player_in_lobby_initial_connect(connection, room_code);

                log!(important "Lobby"; "Created {room_code}");
            },
            ToServerPacket::Leave => {
                self.set_player_outside_lobby(connection.get_address(), false);
            },
            ToServerPacket::Kick { player_id: kicked_player_id } => {
                let Some(host_location) = self.clients
                    .get(connection.get_address())
                    .map(|p|&p.location)
                else{
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from unconnected player!", connection.get_address());
                    return Ok(());
                };

                let ListenerClientLocation::InLobby{room_code, lobby_client_id: host_id} = host_location else {
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from player not in a lobby!", connection.get_address());
                    return Ok(());
                };

                if let Some(lobby) = self.lobbies.get_mut(room_code){
                    if !lobby.is_host(*host_id) {return Ok(());}

                    let kicked_player = self.get_address_from_location(ListenerClientLocation::InLobby { room_code: *room_code, lobby_client_id: kicked_player_id });
                    if let Some(kicked_player_address) = kicked_player {
                        if let Some(connection) = self.clients.get(&kicked_player_address).map(|p|p.connection.clone()) {
                            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                            self.set_player_outside_lobby(&kicked_player_address, false);
                        }
                    }else{
                        //Nobody is connected to that lobby with that id,
                        //Maybe they already left

                        if let Some(lobby) = self.lobbies.get_mut(room_code){
                            lobby.remove_player(kicked_player_id);
                        }
                    }
                }
            },
            _ => {
                let Some(sender_player_location) = self.clients
                    .get_mut(connection.get_address())
                    .map(|p|&mut p.location)
                else{
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from unconnected player!", connection.get_address());
                    return Ok(());
                };

                if let ListenerClientLocation::InLobby { room_code, lobby_client_id } = sender_player_location {
                    if let Some(lobby) = self.lobbies.get_mut(room_code){
                        lobby.on_client_message(&connection.get_sender(), *lobby_client_id, incoming_packet);
                    } else {
                        //Player is in a lobby that doesn't exist
                        panic!("Recieved a message from a player in a lobby that doesnt exist")
                    }
                }
            }
        }
    
        Ok(())
    }
}