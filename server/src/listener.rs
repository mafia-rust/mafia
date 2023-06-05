use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::Duration};

use rand::random;
use tokio_tungstenite::tungstenite::Message;

use crate::{lobby::Lobby, log, websocket_connections::connection::Connection, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}};

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
                        println!("{}\t{}", log::important("LOBBY CLOSED:"), key);
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
        println!("{}\t{}", log::important("CONNECTED:"), connection.get_address());

        self.players.insert(*connection.get_address(), PlayerLocation::OutsideLobby);
    }

    pub fn on_disconnect(&mut self, connection: &Connection) {
        println!("{}\t{}", log::important("DISCONNECTED:"), connection.get_address());

        if let Some(disconnected_player) = self.players.remove(connection.get_address()) {
            if let PlayerLocation::InLobby { room_code, player_id } = disconnected_player {
                // If the lobby actually exists
                if let Some(lobby) = self.lobbies.get_mut(&room_code){
                    lobby.disconnect_player_from_lobby(player_id);
                }
            }
        } else {
            println!("{} {}", log::error("Tried to disconnect a non existent player!"), connection.get_address())
        }
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() {
            return; // They either disconnected, or sent nothing.
        }
        println!("[{}]\t{}", log::notice(&connection.get_address().to_string()), message);

        if let Err(k) = self.handle_message(connection, message){
            println!("[{}]\t{}:\n{}", log::error(&connection.get_address().to_string()), log::error("SERDE ERROR"), k);
        }    
    }
    fn handle_message(&mut self, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {
        let incoming_packet = serde_json::from_str::<ToServerPacket>(message.to_string().as_str())?;

        let Some(sender_player_location) = self.players.get_mut(connection.get_address()) else {
            println!("{} {}", log::error("Received packet from unconnected player!"), connection.get_address());
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
        
                        connection.send(ToClientPacket::AcceptJoin);
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

                println!("{}\t{}", log::important("LOBBY CREATED:"), room_code);

                self.lobbies.insert(room_code, lobby);
            },
            _ => {
                if let PlayerLocation::InLobby { room_code, player_id } = sender_player_location {
                    if let Some(lobby) = self.lobbies.get_mut(room_code){
                        lobby.on_client_message(connection, *player_id, incoming_packet);
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