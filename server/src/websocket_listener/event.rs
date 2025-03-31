use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

use crate::{log, packet::ToServerPacket, websocket_connections::connection::Connection, websocket_listener::ValidateClientError};

use super::WebsocketListener;

impl WebsocketListener{
    pub fn on_connect(&mut self, connection: &Connection) {
        self.create_client(connection);
    }

    pub fn on_disconnect(&mut self, connection: Connection) {
        self.delete_client(connection.get_address());
    }

    pub fn on_message(&mut self, connection: &Connection, message: &Message) {
        if message.is_empty() { return }

        log!(info "listener.rs"; "{}: {}", &connection.get_address().to_string(), message);

        let Ok(packet) = serde_json::from_str::<ToServerPacket>(message.to_string().as_str()) else {
            log!(error "listener.rs"; "Recieved message but could not parse packet");
            return
        };

        match self.valid_client(connection.get_address()) {
            Err(ValidateClientError::ClientDoesntExist) =>
                log!(error "listener.rs"; "Received packet from an address with no client"),
            Err(ValidateClientError::InLobbyThatDoesntExist) => 
                log!(error "listener.rs"; "Received packet from a client in a lobby that doesnt exist"),
            Ok(()) => ()
        }

        self.handle_message(connection, packet)
    }
    pub(super) fn tick(&mut self, delta_time: Duration){
        let mut closed_lobbies = Vec::new();
        let mut closed_clients = Vec::new();
                    
        let Self { ref mut lobbies, ref mut clients} = *self;

        // log!(info "Listener"; "lobbies: {:?} players: {:?}", lobbies.keys(), _players.len());

        for (room_code, lobby) in lobbies.iter_mut() {
            if lobby.is_closed() {
                closed_lobbies.push(*room_code);
            } else {
                lobby.tick(delta_time);
            }
        }

        for (client_address, listener_client) in clients.iter_mut(){
            listener_client.tick();
            if listener_client.ping_timed_out() {
                closed_clients.push(*client_address);
            }
        }

        for key in closed_lobbies {
            log!(important "Lobby"; "Closed {key} due to lobby closed");
            self.delete_lobby(key);
        }
        for key in closed_clients {
            log!(important "Connection"; "Closed {key} due to ping timed out");
            let _ = self.delete_client(&key);
        }
    }
}