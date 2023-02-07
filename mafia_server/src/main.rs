
use mafia_server::{lobby::Lobby, network::websocket_listener::create_ws_server};
use mafia_server::network::connection::{Connection, ConnectionEventListener};
use tokio_tungstenite::tungstenite::Message;
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

    // let mut lobbies = HashMap::new();
    // lobbies.insert("0", Lobby::new());

    let listener = Listener {
        lobby: Lobby::new()
    };

    let server_future = create_ws_server("127.0.0.1:8081", clients, Box::new(listener));

    server_future.await;

    Ok(())
}

struct Listener {
    lobby: Lobby,
}

impl ConnectionEventListener for Listener {
    fn on_connect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("connected: {}", connection.get_address());
    }

    fn on_disconnect(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection) {
        println!("disconnected: {}", connection.get_address());
    }

    fn on_message(&mut self, _clients: &HashMap<SocketAddr, Connection>, connection: &Connection, message: &Message) {
        println!("{}: {}", message, connection.get_address());
    }
}

// TODO: @ItsSammyM Move this, also use for room codes
///
/// Converts x to any radix
/// # Example
/// ```
/// assert_eq!(format_radix(7, 2), "111");
/// assert_eq!(format_radix(366, 10), "366");
/// assert_eq!(format_radix(36*36*36*36 - 1, 36), "zzzz");
/// ```
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // TODO: @Jack-Papel Change to return result
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
