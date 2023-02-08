
use mafia_server::lobby;
use mafia_server::{lobby::Lobby, network::websocket_listener::create_ws_server};
use mafia_server::network::connection::{Connection, ConnectionEventListener};
use mafia_server::network::packet::{ToClientPacket, ToServerPacket};
use mafia_server::game::Game;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;
use std::hash::Hash;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
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

    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = Listener::new();

    let server_future = create_ws_server("127.0.0.1:8081", clients, Box::new(listener));

    server_future.await;

    Ok(())
}

struct Listener {
    lobby: Lobby,
    player_ids: HashMap<SocketAddr, usize>,
}
impl Listener{
    fn new()->Self{
        Self{
            lobby: Lobby::new(),
            player_ids: HashMap::new(),
        }
    }
}
impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("connected: {}", connection.get_address());
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("disconnected: {}", connection.get_address());
    }

    fn on_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) {
        println!("{}, addr:{}", message, connection.get_address());

        let json_value = json!(message.to_string());
        let incoming_packet = serde_json::value::from_value::<ToServerPacket>(json_value);

        match incoming_packet{
            Ok(incoming_packet) => {
                match incoming_packet {
                    ToServerPacket::Join => {
                        connection.send(
                            ToClientPacket::AcceptJoin
                        );

                        let mut max_id = 0usize;
                        for id in self.player_ids.values().into_iter(){
                            max_id = id.clone();
                        }
                        self.player_ids.insert(connection.get_address().clone(), max_id+1);

                    },
                    ToServerPacket::Host => {
                        connection.send(
                            ToClientPacket::AcceptHost{
                                room_code: "temp_room_code".to_string(),
                            }
                        );

                        self.player_ids.insert(connection.get_address().clone(), 1);

                    },
                    _ => {
                        println!("Unhandled incoming_packet: {:?}", incoming_packet);
                        todo!();
                    }
                }
            
            
            },
            Err(_err)=>{
                //println!("Json failed to parse: {}", err)
            },
        }
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
