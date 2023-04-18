use std::{net::SocketAddr, collections::HashMap, sync::{Mutex, Arc}, time::{Duration, SystemTime, Instant}};

use serde_json::Value;
use tokio_tungstenite::tungstenite::{Message, protocol::frame};

use crate::{lobby::Lobby, log};

use super::{connection::{ConnectionEventListener, Connection}, packet::{ToServerPacket, ToClientPacket, RejectJoinReason}};

pub type ArbitraryPlayerID = u32;
pub type RoomCode = usize;


pub struct Listener {
    lobbies: Arc<Mutex<HashMap<RoomCode, Lobby>>>,
    players: HashMap<SocketAddr, Option<(RoomCode, ArbitraryPlayerID)>>,
    /*
    IP->ArbitraryID->Playerindex
    IP->RoomCode
*/
}
impl Listener{
    pub fn new()->Self{
        let out = Self{
            lobbies: Arc::new(Mutex::new(HashMap::new())),
            players: HashMap::new(),
        };

        let threaded_lobbies = out.lobbies.clone();
        let frame_period = Duration::from_secs(1);

        tokio::spawn(async move {
            let mut last_tick = tokio::time::Instant::now();
            loop {
                let dt = last_tick.elapsed();
                for (_, lobby) in threaded_lobbies.lock().unwrap().iter_mut(){
                    lobby.tick(dt);
                }

                last_tick = tokio::time::Instant::now();
                if let Some(sleep_time) = frame_period.checked_sub(dt) {
                    tokio::time::sleep(frame_period).await;
                } // Else, the last tick took super long. Don't wait any more.
            }
        });
        out
    }
}

impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("{}\t{}", log::important("CONNECTED:   "), connection.get_address());

        //add player
        self.players.insert(connection.get_address().clone(), None);
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("{}\t{}", log::important("DISCONNECTED:"), connection.get_address());

        //remove player
        self.players.remove(connection.get_address());
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
                        Ok(player_arbitrary_id) => {
                            *player = Some((room_code, player_arbitrary_id));
        
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

                //TODO
                //Find unused room code
                let new_room_code: RoomCode = 0;

                //Make sure there are no players who have joined the game under this roomcode, If so, send them back to startmenu and remove them from lobby
                for (addr, route_opt) in self.players.iter_mut(){
                    if let Some(route) = route_opt{
                        if route.0 == new_room_code{
                            *route_opt = None;  //remove from lobby
                            clients.get(addr).unwrap().send(ToClientPacket::Kicked { reason: "The lobby your connected to no longer exists".to_string() }); //send back to start
                        }
                    }
                }

                let mut lobby = Lobby::new();
                
                if let Some(player) = self.players.get_mut(connection.get_address()){
                    match lobby.join_player(connection.get_sender()) {
                        Ok(player_index) => {
                            *player = Some((new_room_code, player_index));
        
                            connection.send(
                                ToClientPacket::AcceptHost{
                                    room_code: new_room_code.to_string(),
                                }
                            );
                        },
                        Err(reason) => {
                            connection.send(ToClientPacket::RejectJoin { reason });
                        }
                    }
                }

                self.lobbies.lock().unwrap().insert(new_room_code, lobby);
            },
            _ => {
                if let Some(player) = self.players.get_mut(connection.get_address()){   //if the player exists
                    if let Some( (room_code, arbitrary_player_id) ) = player {               //if the player claims to be in a lobby
                        if let Some(lobby) = self.lobbies.lock().unwrap().get_mut(room_code){          //if the lobby that player is in exists
                            lobby.on_client_message(connection.get_sender(), arbitrary_player_id.clone(), incoming_packet);
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


