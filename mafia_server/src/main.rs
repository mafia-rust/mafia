
use mafia_server::{lobby::Lobby, network::websocket_listener::create_ws_server};
use mafia_server::network::connection::{Connection, ConnectionEventListener};
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
};


#[tokio::main]
async fn main()->Result<(), ()>{
    println!("Hello, world!");

    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut lobbies = HashMap::new();
    lobbies.insert("0", Lobby::new());

    let listener = Listener{
        lobbies
    };

    let server_future = create_ws_server("127.0.0.1:8081", clients, Box::new(listener));

    server_future.await;
    return Ok(());
}
struct Listener{
    lobbies: HashMap<&'static str, Lobby>,
}
impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("connected: {}", connection.get_adress());
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("disconnected: {}", connection.get_adress());
    }

    fn on_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) {
        println!("{}: {}", message, connection.get_adress());
    }
}







//use this for room codes


/**
Converts x to any radix
# Panics
radix < 2 || radix > 36
# Example
```
format_radix(7, 2) == "111";
format_radix(366, 10) == "366";
format_radix(36*36*36*36 - 1, 36) == "zzzz";
```
*/
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
