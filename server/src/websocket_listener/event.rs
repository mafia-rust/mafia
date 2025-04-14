use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

use crate::{log, packet::ToServerPacket, websocket_connections::connection::Connection};

use super::{client::ClientReference, WebsocketListener, ValidateClientError};

impl WebsocketListener{
    pub fn on_connect(&mut self, connection: &Connection) {
        self.create_client(connection);
    }

    pub fn on_disconnect(&mut self, connection: Connection) {
        if let Some(client) = ClientReference::new(connection.address(), self){
            self.delete_client(&client);
        }
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() { return }

        log!(info "listener.rs"; "{}: {}", &connection.address().to_string(), message);

        let Ok(packet) = serde_json::from_str::<ToServerPacket>(message.to_string().as_str()) else {
            log!(error "listener.rs"; "Recieved message but could not parse packet");
            return
        };

        match self.validate_client(connection.address()) {
            Err(ValidateClientError::ClientDoesntExist) =>
                log!(error "listener.rs"; "Received packet from an address with no client"),
            Err(ValidateClientError::InLobbyThatDoesntExist) => 
                log!(error "listener.rs"; "Received packet from a client in a lobby that doesnt exist"),
            Ok(client) => {
                self.handle_message(client, packet)
            }
        }
    }
    pub(super) fn tick(&mut self, delta_time: Duration){
        let mut closed_lobbies = Vec::new();
        let mut closed_clients = Vec::new();

        for (room_code, lobby) in self.lobbies_mut().iter_mut() {
            let tick_data = lobby.tick(delta_time);
            if tick_data.close_room {
                closed_lobbies.push(*room_code);
            }
        }

        for client in ClientReference::all_clients(self){
            client.tick(self);
            if client.ping_timed_out(self) {
                closed_clients.push(client);
            }
        }

        for room_code in closed_lobbies {
            log!(important "Lobby"; "Closed {room_code} due to lobby closed");
            self.delete_lobby(room_code);
        }
        for client in closed_clients {
            log!(important "Connection"; "Closed {} due to ping timed out", client.address(self));
            self.delete_client(&client);
        }
    }
}