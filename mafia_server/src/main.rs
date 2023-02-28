
use mafia_server::{
    lobby::Lobby,
    lobby::LobbyIndex,
    network::{
        websocket_listener::create_ws_server,
        connection::{Connection, ConnectionEventListener},
        packet::{ToClientPacket, ToServerPacket}
    },
    game::{player::PlayerIndex}
};
// use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, 
    collections::HashMap,
};
/*
#[derive(Serialize, Deserialize)]
enum Test{
    Amongus1(i8),
    Amongus(i8, bool),
    Tos,
    Mafia{lol: bool}
}
{
    "Amongus1": 6
}
{
    "Amongus": [
        6,
        true
    ]
}
"Tos"
{
    "Mafia": {
        "lol": false
    }
}
*/
///
/// The Main function
/// 
/// # Examples
/// ![image](https://user-images.githubusercontent.com/64770632/217148805-aa33cad8-f1b8-45ff-954c-c57e5fdb54c9.png)
/// 
#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");    
    // println!("{}", serde_json::to_string_pretty(&Test::Amongus1(6)).unwrap());
    // println!("{}", serde_json::to_string_pretty(&Test::Amongus(6, true)).unwrap());
    // println!("{}", serde_json::to_string_pretty(&Test::Tos).unwrap());
    // println!("{}", serde_json::to_string_pretty(&Test::Mafia{lol: false}).unwrap());


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

                        let player_index = lobby.add_new_player(connection.get_sender());

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

                    let player_index = lobby.add_new_player(connection.get_sender());
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


