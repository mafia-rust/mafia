
use mafia_server::{websocket_connections::websocket_listener::create_ws_server, log};
use serde::Deserialize;
use std::{fs, thread, time::Duration};

#[derive(Deserialize)]
struct Config{
    address: String,
}

///
/// The Main function
/// 
/// # Examples
/// ![image](https://user-images.githubusercontent.com/64770632/217148805-aa33cad8-f1b8-45ff-954c-c57e5fdb54c9.png)
/// 
#[tokio::main]
async fn main() {
    let config = serde_json::from_str::<Config>(
        &fs::read_to_string("./resources/config.json").expect("Failed to read the config file")
    ).unwrap();

    loop {
        create_ws_server(&config.address).await;
        // This delay is only to make sure disconnect messages are sent before the server restarts
        thread::sleep(Duration::from_secs(1));
        log!(important "Main"; "Restarting server...");
    }
}
