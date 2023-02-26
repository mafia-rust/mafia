
use mafia_server::{
    lobby::Lobby,
    lobby::LobbyIndex,
    network::{
        websocket_listener::create_ws_server,
        connection::{Connection, ConnectionEventListener},
        packet::{ToClientPacket, ToServerPacket}
    },
    game::{player::PlayerIndex, settings::{Settings}}
};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, 
    collections::HashMap,
};


///
/// The Main function
/// 
/// # Examples
/// ![image](https://user-images.githubusercontent.com/64770632/217148805-aa33cad8-f1b8-45ff-954c-c57e5fdb54c9.png)
/// 
#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");    
    let p = serde_json::to_string_pretty(&Settings::new(1));
    println!("{}", p.unwrap());


    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = Listener::new();

    let server_future = create_ws_server("127.0.0.1:8081", clients, Box::new(listener));
    
    server_future.await;

    Ok(())
}

struct Listener {
    lobbies: Vec<Lobby>,
    players: HashMap<SocketAddr, Option<(LobbyIndex, PlayerIndex)>>,
}
impl Listener{
    fn new()->Self{
        Self{
            lobbies: Vec::new(),
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
            ToServerPacket::Join{ lobby_index } => {
                
                //add to lobby
                if let Some(player) = self.players.get_mut(connection.get_address()){

                    if let Some(lobby) = self.lobbies.get_mut(lobby_index){

                        let player_index = lobby.add_new_player((connection.get_sender(), "".to_string()));

                        *player = Some((lobby_index, player_index));

                        connection.send(ToClientPacket::AcceptJoin);
                    }else{
                        connection.send(ToClientPacket::RejectJoin { reason: "Lobby does not exist".to_string() });
                    }
                }else{
                    connection.send(ToClientPacket::RejectJoin { reason: "Player does not exist".to_string() });
                }
            },
            ToServerPacket::Host => {

                if let Some(player) = self.players.get_mut(connection.get_address()){
                    let mut lobby = Lobby::new();

                    let player_index = lobby.add_new_player((connection.get_sender(), "".to_string()));
                    self.lobbies.push(lobby);

                    *player = Some((self.lobbies.len() - 1, player_index));

                    connection.send(
                        ToClientPacket::AcceptHost{
                            room_code: "temp_room_code".to_string(),
                        }
                    );
                }
            },
            _ => {
                if let Some(player) = self.players.get_mut(connection.get_address()){
                    if let Some( (lobby_index, player_index) ) = player {
                        
                        self.lobbies.get_mut(*lobby_index).unwrap()
                            .on_client_message(connection.get_sender(), *player_index, incoming_packet);

                    }
                }
            }
        }
    
        Ok(())
    }
}



//use this for room codes


/**
Converts x to any radix
# Panics
radix < 2 || radix > 36
# Example
```
assert_eq!(format_radix(7, 2), "111");
assert_eq!(format_radix(366, 10), "366");
assert_eq!(format_radix(36*36*36*36 - 1, 36), "zzzz");
```
*/
#[allow(unused)]
fn format_radix(mut x: u32, radix: u32) -> Option<String> {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;
        
        result.push(std::char::from_digit(m, radix)?);
        if x == 0 {
            break;
        }
    }
    Some(result.into_iter().rev().collect())
}


