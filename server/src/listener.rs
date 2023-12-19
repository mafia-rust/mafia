use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::Duration};

use rand::random;
use tokio_tungstenite::tungstenite::Message;

use crate::{lobby::Lobby, websocket_connections::connection::Connection, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}, log};

pub type PlayerID = u32;
pub type RoomCode = usize;

enum PlayerLocation {
    InLobby {
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
                    
                    

                    let Listener { ref mut lobbies, ref mut players } = *listener;

                    for (room_code, lobby) in lobbies.iter_mut() {
                        if lobby.is_closed() {
                            closed_lobbies.push(*room_code);
                        } else {
                            lobby.tick(delta_time);





                            //TODO move this somewhere else, 
                            //Kick players from lobby and disconnect them from listener
                            lobby.get_players_to_kick().into_iter().for_each(|id|{
                                if let Some(address) = Self::get_address_from_id(players, *room_code, id){
                                    players.remove(&address);
                                    lobby.disconnect_player_from_lobby(id);
                                }
                            });
                        }
                    }

                    for key in closed_lobbies {
                        log!(important "Lobby"; "Closed {key}");
                        listener.lobbies.remove(&key);
                    }
                } else { 
                    return;
                }

                tokio::time::sleep(DESIRED_FRAME_TIME.saturating_sub(tokio::time::Instant::now() - frame_start_time)).await;
            }
        });
    }

    fn get_address_from_id(players: &HashMap<SocketAddr, PlayerLocation>, room_code: RoomCode, id: PlayerID)->Option<SocketAddr>{
        players.iter().find(|p|
            if let PlayerLocation::InLobby{room_code: player_room_code, player_id} = p.1{
                *player_id == id && room_code == *player_room_code
            }else{
                false
            }
        ).map(|p| *p.0)
    }

    pub fn on_connect(&mut self, connection: &Connection) {
        self.players.insert(*connection.get_address(), PlayerLocation::OutsideLobby);
    }

    // TODO: DisconnectError enum
    pub fn on_disconnect(&mut self, connection: Connection) -> Result<(), &'static str> {
        let Some(disconnected_player_location) = self.players.remove(connection.get_address()) else {
            return Err("Player doesn't exist");
        };

        let PlayerLocation::InLobby { room_code, player_id } = disconnected_player_location else {
            return Err("Player is not in a lobby, but was removed from listener");
        };

        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            return Err("Player is in a lobby that doesn't exist, but was removed from listener");
        };

        lobby.disconnect_player_from_lobby(player_id);
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

        let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            return Ok(());
        };

        match incoming_packet {
            ToServerPacket::Join{ room_code } => {
                let Some(lobby) = self.lobbies.get_mut(&room_code) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::InvalidRoomCode });
                    return Ok(());
                };

                match lobby.connect_player_to_lobby(&connection.get_sender()) {
                    Ok(player_id) => {
                        *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
                    },
                    Err(reason) => {
                        connection.send(ToClientPacket::RejectJoin { reason });
                    }
                }
            },
            ToServerPacket::Host => {
                let Some(room_code) = ((random::<u16>() as usize)..usize::MAX).find(
                    //TODO make this fill up the usize entirely
                    |code| !self.lobbies.contains_key(code)
                ) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };

                let mut lobby = Lobby::new(room_code);
                
                match lobby.connect_player_to_lobby(&connection.get_sender()) {
                    Ok(player_id) => {
                        *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
        
                        // connection.send(ToClientPacket::AcceptHost{ room_code, player_id });
                    },
                    Err(reason) => {
                        connection.send(ToClientPacket::RejectJoin { reason });
                    }
                }

                log!(important "Lobby"; "Created {room_code}");

                self.lobbies.insert(room_code, lobby);
            },
            _ => {
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