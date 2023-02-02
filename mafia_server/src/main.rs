
use mafia_server::{lobby::Lobby, network::websocket_listener::create_ws_server};
use mafia_server::network::connection::Connection;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, collections::HashMap,
};


#[tokio::main]
async fn main()->Result<(), ()>{


    println!("Hello, world!");

    let mut lobbies = HashMap::new();
    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let server_future = create_ws_server("127.0.0.1:8081", clients);

    lobbies.insert("0", Lobby::new());
    let lobby = lobbies.get("0").expect("lobby 0 should be set by previous line");

    server_future.await;
    return Ok(());
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
