
use mafia_server::{
    network::{
        websocket_listener::create_ws_server,
        connection::{Connection},
        listener::Listener
    },
};
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

    let clients: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = Listener::new();

    let server_future = create_ws_server("127.0.0.1:8081", clients, Box::new(listener));
    
    server_future.await;

    Ok(())
}



