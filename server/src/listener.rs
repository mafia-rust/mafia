use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::Duration};

use rand::random;
use tokio_tungstenite::tungstenite::Message;

use crate::{lobby::Lobby, websocket_connections::connection::Connection, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}, log};

pub type PlayerID = u32;
pub type RoomCode = usize;

enum PlayerLocation {
    InLobby{
        room_code: RoomCode,
        player_id: PlayerID,
    },
    OutsideLobby
}

pub struct Listener {
    lobbies: HashMap<RoomCode, Lobby>,
    players: HashMap<SocketAddr, PlayerLocation>,
}
impl Listener{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            players: HashMap::new(),
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
                    
                    

                    let Listener { ref mut lobbies, .. } = *listener;

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
            //TODO make this fill up the usize entirely
            |code| !self.lobbies.contains_key(code)
        ) else {
            return None;
        };

        let lobby = Lobby::new(room_code);
        self.lobbies.insert(room_code, lobby);
        Some(room_code)
    }
    fn delete_lobby(&mut self, room_code: RoomCode){
        for player in self.players.iter_mut()
            .filter(|p| 
                if let PlayerLocation::InLobby{room_code: player_room_code, ..} = *p.1 {
                    player_room_code == room_code 
                }else{
                    false
                }
            )
        {
            *player.1 = PlayerLocation::OutsideLobby;
        }
        self.lobbies.remove(&room_code);
    }

    fn connect_player_to_lobby(&mut self, connection: &Connection, room_code: RoomCode){
        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        match lobby.connect_player_to_lobby(&connection.get_sender()) {
            Ok(player_id) => {
                *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
                return;
            },
            Err(_) => {
                return;
            }
        }
    }
    fn reconnect_player_to_lobby(&mut self, connection: &Connection, room_code: RoomCode, player_id: PlayerID){

        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        match lobby.reconnect_player_to_lobby(&connection.get_sender(), player_id) {
            Ok(_) => {
                *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    // fn get_address_from_id(players: &HashMap<SocketAddr, PlayerLocation>, room_code: RoomCode, id: PlayerID)->Option<SocketAddr>{
    //     players.iter().find(|p|
    //         if let PlayerLocation::InLobby{room_code: player_room_code, player_id} = p.1{
    //             *player_id == id && room_code == *player_room_code
    //         }else{
    //             false
    //         }
    //     ).map(|p| *p.0)
    // }




    pub fn on_connect(&mut self, connection: &Connection) {
        self.players.insert(*connection.get_address(), PlayerLocation::OutsideLobby);
    }

    // TODO: DisconnectError enum
    pub fn on_disconnect(&mut self, connection: Connection) -> Result<(), &'static str> {
        let Some(disconnected_player_location) = self.players.remove(connection.get_address()) else {
            return Err("Player doesn't exist");
        };

        if let PlayerLocation::InLobby { room_code, player_id } = disconnected_player_location {
            if let Some(lobby) = self.lobbies.get_mut(&room_code) {
                lobby.lose_player_connection(player_id);
            }
        }

        Ok(())
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() { return }

        log!(info "Listener"; "{}: {}", &connection.get_address().to_string(), message);
        if let Err(k) = self.handle_message(connection, message){
            log!(error "Listener"; "Serde error when receiving message from {}: {}", &connection.get_address().to_string(), k);
        }
    }
    // TODO sum the error types in this function so they can be handled in on_message
    fn handle_message(&mut self, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {
        let incoming_packet = serde_json::from_str::<ToServerPacket>(message.to_string().as_str())?;

        match incoming_packet {
            ToServerPacket::LobbyListRequest => {
                let lobbies = self.lobbies.iter().map(|(room_code, _lobby)| {
                    *room_code
                }).collect::<Vec<_>>();

                connection.send(ToClientPacket::LobbyList { room_codes: lobbies });
            },
            ToServerPacket::ReJoin {room_code, player_id } => {
                self.reconnect_player_to_lobby(connection, room_code, player_id);
            }
            ToServerPacket::Join{ room_code } => {
                self.connect_player_to_lobby(connection, room_code);
            },
            ToServerPacket::Host => {
                let Some(room_code) = self.create_lobby() else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };
                
                self.connect_player_to_lobby(connection, room_code);

                log!(important "Lobby"; "Created {room_code}");
            },
            ToServerPacket::Leave => {
                let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from unconnected player!", connection.get_address());
                    return Ok(());
                };

                if let PlayerLocation::InLobby { room_code, player_id } = sender_player_location {
                    if let Some(lobby) = self.lobbies.get_mut(room_code){
                        lobby.on_client_message(&connection.get_sender(), *player_id, incoming_packet);
                    } else {
                        //Player is in a lobby that doesn't exist
                        panic!("Recieved a message from a player in a lobby that doesnt exist")
                    }
                }
                *sender_player_location = PlayerLocation::OutsideLobby;
            },
            _ => {
                let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from unconnected player!", connection.get_address());
                    return Ok(());
                };

                if let PlayerLocation::InLobby { room_code, player_id } = sender_player_location {
                    if let Some(lobby) = self.lobbies.get_mut(room_code){
                        lobby.on_client_message(&connection.get_sender(), *player_id, incoming_packet);
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