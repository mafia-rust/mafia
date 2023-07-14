use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::Duration};

use rand::random;
use tokio_tungstenite::tungstenite::Message;

use crate::{lobby::Lobby, websocket_connections::connection::{Connection}, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}, log};

// TODO, rename to PregameID or IntermediaryID
pub type ArbitraryPlayerID = u32;
pub type RoomCode = usize;

enum PlayerLocation {
    InLobby {
        room_code: RoomCode,
        player_id: ArbitraryPlayerID,
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
        const DESIRED_FRAME_TIME: Duration = Duration::from_millis(980);

        tokio::spawn(async move {
            let mut last_tick = tokio::time::Instant::now();
            loop {
                let delta_time;
                { // Tick, and remove completed lobbies
                    let Ok(mut listener) = listener.lock() else {return};
                    let mut closed_lobbies = Vec::new();
                    
                    delta_time = last_tick.elapsed();
                    last_tick = tokio::time::Instant::now();

                    for (key, lobby) in listener.lobbies.iter_mut() {
                        if lobby.is_closed() {
                            closed_lobbies.push(*key);
                        } else {
                            lobby.tick(delta_time);
                        }
                    }

                    // Remove closed lobbies
                    for key in closed_lobbies {
                        log!(important "Lobby"; "Closed {key}");
                        listener.lobbies.remove(&key);
                    }
                }

                // Calculate sleep time based on the last frame time
                tokio::time::sleep(DESIRED_FRAME_TIME).await;
            }
        });
    }
}

impl Listener {
    pub fn on_connect(&mut self, connection: &Connection) {
        self.players.insert(*connection.get_address(), PlayerLocation::OutsideLobby);
    }

    // TODO: DisconnectError enum
    pub fn on_disconnect(&mut self, connection: Connection) -> Result<(), &'static str> {
        let Some(disconnected_player_location) = self.players.remove(connection.get_address()) else {
            return Err("Player doesn't exist");
        };

        let PlayerLocation::InLobby { room_code, player_id } = disconnected_player_location else {
            return Err("Player is not in a lobby");
        };

        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            return Err("Player is in a lobby that doesn't exist");
        };

        lobby.disconnect_player_from_lobby(player_id);
        Ok(())
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() {
            return; // They either disconnected, or sent nothing.
        }
        log!(info "Listener"; "{}: {}", &connection.get_address().to_string(), message);

        if let Err(k) = self.handle_message(connection, message){
            log!(error "Listener"; "Serde error when recieving message from {}: {}", &connection.get_address().to_string(), k);
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
        
                        // connection.send(ToClientPacket::AcceptJoin{in_game: false});
                    },
                    Err(reason) => {
                        connection.send(ToClientPacket::RejectJoin { reason });
                    }
                }
            },
            ToServerPacket::Host => {
                let Some(room_code) = ((random::<u16>() as usize)..usize::MAX).find(
                    |code| !self.lobbies.contains_key(code)
                ) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };

                let mut lobby = Lobby::new();
                
                match lobby.connect_player_to_lobby(&connection.get_sender()) {
                    Ok(player_id) => {
                        *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
        
                        connection.send(ToClientPacket::AcceptHost{ room_code });
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
                        //Player is in a lobby that doesnt exist   
                        todo!()
                    }
                }
            }
        }
    
        Ok(())
    }
}