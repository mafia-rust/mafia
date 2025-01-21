use std::fs;
use serde::Deserialize;
use mongodb::Client;
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use mongodb::bson;

use crate::bson::doc;

mod model;
use model::user::User;

#[derive(Deserialize)]
struct Config {
    amqp_address: String,
    mongodb_uri: String,
    database_name: String,
}

#[tokio::main]
async fn main() {
    let config = serde_json::from_str::<Config>(
        &fs::read_to_string("./resources/config.json").expect("Failed to read the config file")
    ).unwrap();

    let client = Client::with_uri_str(config.mongodb_uri).await.expect("Failed to connect to MongoDB");

    let options = IndexOptions::builder()
        .unique(true)
        .build();
    
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();

    client.database(&config.database_name)
        .collection::<User>("users")
        .create_index(model)
        .await
        .expect("uhh");
}
