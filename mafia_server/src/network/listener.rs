use std::{net::SocketAddr, collections::HashMap};

use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use crate::lobby::Lobby;

use super::{connection::{ConnectionEventListener, Connection}, packet::{ToServerPacket, ToClientPacket}};




pub type ArbitraryPlayerID = u32;
pub type RoomCode = usize;


pub struct Listener {
    lobbies: HashMap<RoomCode, Lobby>,
    players: HashMap<SocketAddr, Option<(RoomCode, ArbitraryPlayerID)>>,
    /*
    IP->ArbitraryID->Playerindex
    IP->RoomCode
*/
}
impl Listener{
    pub fn new()->Self{
        Self{
            lobbies: HashMap::new(),
            players: HashMap::new(),
        }
    }
}

impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("connected: {}", connection.get_address());

        //add player
        self.players.insert(connection.get_address().clone(), None);
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("disconnected: {}", connection.get_address());

        //remove player
        self.players.remove(connection.get_address());
    }

    fn on_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) {
        println!("{}, addr:{}", message, connection.get_address());

        if let Err(k) = self.handle_message(_clients, connection, message){
            println!("Error: {}", k);
        }    
    }
}
impl Listener{
    fn handle_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) -> Result<(), serde_json::Error> {

        let json_value = serde_json::from_str::<Value>(message.to_string().as_str())?;
        let incoming_packet = serde_json::value::from_value::<ToServerPacket>(json_value.clone())?;

        match incoming_packet {
            ToServerPacket::Join{ room_code } => {
                
                //add to lobby
                if let Some(player) = self.players.get_mut(connection.get_address()){
                    if let Some(lobby) = self.lobbies.get_mut(&room_code){

                        let player_arbitrary_id: ArbitraryPlayerID = lobby.add_new_player(connection.get_sender());

                        *player = Some((room_code, player_arbitrary_id));

                        connection.send(ToClientPacket::AcceptJoin);
                    }else{
                        
                        connection.send(ToClientPacket::RejectJoin { reason: format!("Lobby does not exist:{}",room_code) });
                    }
                }else{
                    connection.send(ToClientPacket::RejectJoin { reason: format!("Player does not exist:{}",connection.get_address()) });
                }
            },
            ToServerPacket::Host => {

                if let Some(player) = self.players.get_mut(connection.get_address()){
                    let mut lobby = Lobby::new();

                    let player_index = lobby.add_new_player(connection.get_sender());



                    self.lobbies.insert(0, lobby);  //TODO

                    *player = Some((self.lobbies.len() - 1, player_index));

                    connection.send(
                        ToClientPacket::AcceptHost{
                            room_code: (self.lobbies.len() - 1).to_string(),
                        }
                    );
                }
            },
            _ => {
                if let Some(player) = self.players.get_mut(connection.get_address()){
                    if let Some( (room_code, arbitrary_player_id) ) = player {
                        
                        self.lobbies.get_mut(room_code).unwrap()
                            .on_client_message(connection.get_sender(), arbitrary_player_id.clone(), incoming_packet);

                    }
                }
            }
        }
    
        Ok(())
    }
}


