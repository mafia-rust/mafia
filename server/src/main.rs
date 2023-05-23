
use mafia_server::{
    websocket_connections::{
        websocket_listener::create_ws_server,
        connection::{Connection},
        // packet::{ToClientPacket, ToServerPacket}
    }, listener::Listener,
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, 
    collections::HashMap, fs,
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


print!("{}", ToClientPacket::YourVoting { player_index: Some(4) }.to_json_string());
{"YourVoting":{"player_index":4}}

print!("{}", ToClientPacket::YourVoting { player_index: None }.to_json_string());
{"YourVoting":{"player_index":null}}

*/
///
/// The Main function
/// 
/// # Examples
/// ![image](https://user-images.githubusercontent.com/64770632/217148805-aa33cad8-f1b8-45ff-954c-c57e5fdb54c9.png)
/// 
#[tokio::main]
async fn main() -> Result<(), ()> {

    let config_string: String = fs::read_to_string("./resources/config.json").expect("Should have read the config file");

    let config = serde_json::value::from_value::<Config>(
        serde_json::from_str::<Value>(&config_string).unwrap()
    ).unwrap();


    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = Listener::new();

    let address_string = config.local_ip + ":" + &config.port;
    let server_future = create_ws_server(&address_string, clients, Box::new(listener));    
    
    server_future.await;

    Ok(())
}
#[derive(Deserialize)]
struct Config{
    local_ip: String,
    port: String
}
// serde_json::to_string(&self).unwrap()


