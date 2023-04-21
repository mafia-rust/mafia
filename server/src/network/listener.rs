use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::{Duration, SystemTime, Instant}};

use rand::random;
use serde_json::Value;
use tokio_tungstenite::tungstenite::{Message, protocol::frame};

use crate::{lobby::Lobby, log};

use super::{connection::{ConnectionEventListener, Connection}, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}};

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
    lobbies: Arc<Mutex<HashMap<RoomCode, Lobby>>>,
    players: HashMap<SocketAddr, PlayerLocation>,
}
impl Listener{
    pub fn new() -> Self {
        let out = Self {
            lobbies: Arc::new(Mutex::new(HashMap::new())),
            players: HashMap::new(),
        };

        let threaded_lobbies = out.lobbies.clone();
        const DESIRED_FRAME_TIME: Duration = Duration::from_secs(1);

        tokio::spawn(async move {
            let mut last_tick = tokio::time::Instant::now();
            loop {
                let delta_time;
                { // Tick, and remove completed lobbies
                    let mut lobbies = threaded_lobbies.lock().unwrap();
                    let mut closed_lobbies = Vec::new();
                    
                    delta_time = last_tick.elapsed();
                    for (key, lobby) in lobbies.iter_mut() {
                        if lobby.is_closed() {
                            closed_lobbies.push(*key);
                        } else {
                            lobby.tick(delta_time);
                        }
                    }
                    last_tick = tokio::time::Instant::now();

                    // Remove closed lobbies
                    for key in closed_lobbies {
                        println!("{}\t{}", log::important("LOBBY CLOSED:"), key);
                        lobbies.remove(&key);
                    }
                }

                // Calculate sleep time based on the last frame time
                tokio::time::sleep(DESIRED_FRAME_TIME.saturating_sub(delta_time)).await;
            }
        });
        out
    }
}

impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("{}\t{}", log::important("CONNECTED:   "), connection.get_address());

        self.players.insert(connection.get_address().clone(), PlayerLocation::OutsideLobby);
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("{}\t{}", log::important("DISCONNECTED:"), connection.get_address());

        if let Some(disconnected_player) = self.players.remove(connection.get_address()) {
            if let PlayerLocation::InLobby { room_code, player_id } = disconnected_player {
                // If the lobby actually exists
                if let Some(lobby) = self.lobbies.lock().unwrap().get_mut(&room_code){
                    lobby.disconnect_player(player_id);
                }
            }
        } else {
            println!("{} {}", log::error("Tried to disconnect an already disconnected player!"), connection.get_address())
        }
    }

    fn on_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) {
        if message.is_empty() {
            return; // They either disconnected, or sent nothing.
        }
        println!("[{}]\t{}", log::notice(&connection.get_address().to_string()), message);

        if let Err(k) = self.handle_message(_clients, connection, message){
            println!("[{}]\t{}:\n{}", log::error(&connection.get_address().to_string()), log::error("SERDE ERROR"), k);
        }    
    }
}
impl Listener{
    fn handle_message(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {

        let json_value = serde_json::from_str::<Value>(message.to_string().as_str())?;
        let incoming_packet = serde_json::value::from_value::<ToServerPacket>(json_value.clone())?;

        let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
            println!("{} {}", log::error("Received packet from unconnected player!"), connection.get_address());
            return Ok(());
        };

        match incoming_packet {
            ToServerPacket::Join{ room_code } => {
                let mut all_lobbies = self.lobbies.lock().unwrap();
                
                #[cfg(debug_assertions)] 'HANDLE_DEBUG_LOBBY: {
                    const DEBUG_ROOM_CODE: RoomCode = 4426; // "dbg" in base 18
                    
                    if room_code != DEBUG_ROOM_CODE || all_lobbies.contains_key(&DEBUG_ROOM_CODE){
                        // Let player connect normally
                        break 'HANDLE_DEBUG_LOBBY;
                    } 
                    // Player wants to connect to debug lobby, and it doesn't exist.

                    all_lobbies.insert(DEBUG_ROOM_CODE, Lobby::new());
                }

                let Some(lobby) = all_lobbies.get_mut(&room_code) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::InvalidRoomCode });
                    return Ok(());
                };

                match lobby.join_player(connection.get_sender()) {
                    Ok(player_id) => {
                        *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
        
                        connection.send(ToClientPacket::AcceptJoin);
                    },
                    Err(reason) => {
                        connection.send(ToClientPacket::RejectJoin { reason });
                    }
                }
            },
            ToServerPacket::Host => {
                let mut existing_lobbies = self.lobbies.lock().unwrap();

                let Some(room_code) = ((random::<u16>() as usize)..usize::MAX).find(
                    |code| !existing_lobbies.contains_key(code)
                ) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };

                let mut lobby = Lobby::new();
                
                match lobby.join_player(connection.get_sender()) {
                    Ok(player_id) => {
                        *sender_player_location = PlayerLocation::InLobby { room_code, player_id };
        
                        connection.send(ToClientPacket::AcceptHost{ room_code });
                    },
                    Err(reason) => {
                        connection.send(ToClientPacket::RejectJoin { reason });
                    }
                }

                println!("{}\t{}", log::important("LOBBY CREATED:"), room_code);

                existing_lobbies.insert(room_code, lobby);
            },
            _ => {
                if let PlayerLocation::InLobby { room_code, player_id } = sender_player_location {
                    if let Some(lobby) = self.lobbies.lock().unwrap().get_mut(room_code){
                        lobby.on_client_message(connection.get_sender(), player_id.clone(), incoming_packet);
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


