
use mafia_server::websocket_connections::websocket_listener::create_ws_server;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config{
    local_ip: String,
    port: String
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
        &fs::read_to_string("./resources/config.json").expect("Should have read the config file")
    ).unwrap();

    let address_string = config.local_ip + ":" + &config.port;
    
    create_ws_server(&address_string).await
}


///removes multiple whitespace in a row.
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        //if theyre not both spaces, keep the character
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}
///removes multiple whitespace in a row.
pub fn trim_new_line(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        //if theyre not both spaces, keep the character
        let result = ch != '\n' || prev != '\n';
        prev = ch;
        result
    });
    new_str
}
