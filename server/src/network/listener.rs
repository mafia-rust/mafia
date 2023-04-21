use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::{Duration, SystemTime, Instant}};

use rand::random;
use serde_json::Value;
use tokio_tungstenite::tungstenite::{Message, protocol::frame};

use crate::{lobby::Lobby, log};

use super::{connection::{ConnectionEventListener, Connection}, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}};

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
            println!("[{}]\t{}:\n{}", log::error(&connection.get_address().to_string()), log::error("ERROR"), k);
        }    
    }
}
impl Listener{
    fn handle_message(&mut self, clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {

        let json_value = serde_json::from_str::<Value>(message.to_string().as_str())?;
        let incoming_packet = serde_json::value::from_value::<ToServerPacket>(json_value.clone())?;

        match incoming_packet {
            ToServerPacket::Join{ room_code } => {
                let Some(player) = self.players.get_mut(connection.get_address()) else {
                    unreachable!("Player should have been added to the hashmap!");
                };
                if let Some(lobby) = self.lobbies.lock().unwrap().get_mut(&room_code) {
                    match lobby.join_player(connection.get_sender()) {
                        Ok(player_id) => {
                            *player = PlayerLocation::InLobby { room_code, player_id };
        
                            connection.send(ToClientPacket::AcceptJoin);
                        },
                        Err(reason) => {
                            connection.send(ToClientPacket::RejectJoin { reason });
                        }
                    }
                } else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::InvalidRoomCode });
                }
            },
            ToServerPacket::Host => {
                let mut existing_lobbies = self.lobbies.lock().unwrap();

                let Some(lobby_room_code) = ((random::<u16>() as usize)..usize::MAX).find(
                    |code| !existing_lobbies.contains_key(code)
                ) else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return Ok(());
                };

                //Make sure there are no players who have joined the game under this roomcode, If so, send them back to startmenu and remove them from lobby
                for (addr, player_location) in self.players.iter_mut(){
                    if let PlayerLocation::InLobby{ room_code, .. } = player_location {
                        if *room_code == lobby_room_code {
                            *player_location = PlayerLocation::OutsideLobby;
                            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::InvalidRoomCode });
                        }
                    }
                }

                let mut lobby = Lobby::new();
                
                if let Some(player) = self.players.get_mut(connection.get_address()){
                    match lobby.join_player(connection.get_sender()) {
                        Ok(player_id) => {
                            *player = PlayerLocation::InLobby { room_code: lobby_room_code, player_id };
        
                            connection.send(
                                ToClientPacket::AcceptHost{
                                    room_code: lobby_room_code.to_string(),
                                }
                            );
                        },
                        Err(reason) => {
                            connection.send(ToClientPacket::RejectJoin { reason });
                        }
                    }
                }

                println!("{}\t{}", log::important("LOBBY CREATED:"), lobby_room_code);

                existing_lobbies.insert(lobby_room_code, lobby);
            },
            _ => {
                if let Some(player_location) = self.players.get_mut(connection.get_address()){  //if the player exists
                    if let PlayerLocation::InLobby { room_code, player_id } = player_location {    //if the player claims to be in a lobby
                        if let Some(lobby) = self.lobbies.lock().unwrap().get_mut(room_code){   //if the lobby that player is in exists
                            lobby.on_client_message(connection.get_sender(), player_id.clone(), incoming_packet);
                        }else{
                            todo!()
                            //Player is in a lobby that doesnt exist   
                        }
                    }
                }
            }
        }
    
        Ok(())
    }
}


