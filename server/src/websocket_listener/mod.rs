use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};

use client::{Client, ClientLocation, ClientReference, GetLobbyError};
use rand::random;

use crate::{
    lobby::{lobby_client::LobbyClientID, Lobby},
    log,
    packet::{RejectJoinReason, ToClientPacket},
    websocket_connections::connection::Connection
};


mod event;
mod client;
mod handle_message;

pub type RoomCode = usize;

pub struct WebsocketListener {
    // Clients that are currently connected, if a client isnt connected it isnt here
    clients: HashMap<SocketAddr, Client>,
    lobbies: HashMap<RoomCode, Lobby>,
}
impl WebsocketListener{
    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub fn start_tick(listener: Arc<Mutex<Self>>) {
        const DESIRED_FRAME_TIME: Duration = Duration::from_secs(1);

        tokio::spawn(async move {
            let mut frame_start_time = tokio::time::Instant::now();
            loop {
                let delta_time = frame_start_time.elapsed();
                frame_start_time = tokio::time::Instant::now();

                if let Ok(mut listener) = listener.lock() {
                    listener.tick(delta_time);                  
                } else { 
                    return;
                }

                tokio::time::sleep(DESIRED_FRAME_TIME.saturating_sub(tokio::time::Instant::now().saturating_duration_since(frame_start_time))).await;
            }
        });
    }

    fn create_lobby(&mut self) -> Option<RoomCode>{
        let room_code = ((random::<u16>() as usize)..usize::MAX).find(
            |code| !self.lobbies.contains_key(code)
        )?;

        let lobby = Lobby::new(room_code);
        self.lobbies.insert(room_code, lobby);
        Some(room_code)
    }
    fn delete_lobby(&mut self, room_code: RoomCode){
        let clients_to_remove: Vec<_> = self.clients.iter()
            .filter(|p| 
                if let ClientLocation::InLobby{room_code: player_room_code, ..} = p.1.location() {
                    player_room_code == room_code 
                }else{
                    false
                }
            )
            .map(|f|*f.0)
            .collect();

        for client_address in clients_to_remove{
            self.set_player_outside_lobby(&client_address, false);
        }
        self.lobbies.remove(&room_code);
    }

    fn set_player_in_lobby_initial_connect(&mut self, connection: &Connection, room_code: RoomCode){
        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = self.clients
            .get_mut(connection.get_address())
            .map(|p|&mut p.location)
        else{
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        match lobby.join_player(&connection.get_sender()) {
            Ok(lobby_client_id) => {
                *sender_player_location = ListenerClientLocation::InLobby { room_code, lobby_client_id };
        
                connection.send(ToClientPacket::LobbyName { name: lobby.name.clone() })
            }
            Err(reason) => {
                connection.get_sender().send(ToClientPacket::RejectJoin { reason });
            }
        }
    }
    fn set_player_in_lobby_reconnect(&mut self, connection: &Connection, room_code: RoomCode, lobby_client_id: LobbyClientID){

        let Some(lobby) = self.lobbies.get_mut(&room_code) else {
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return;
        };

        let Some(sender_player_location) = 
            self.clients
            .get_mut(connection.get_address())
            .map(|p|&mut p.location)
        else{
            log!(error "Listener"; "{} {}", "Received packet from unconnected player!", connection.get_address());
            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
            return;
        };

        if lobby.rejoin_player(&connection.get_sender(), lobby_client_id).is_ok() {
            *sender_player_location = ListenerClientLocation::InLobby { room_code, lobby_client_id };
        }
        
        connection.send(ToClientPacket::LobbyName { name: lobby.name.clone() })
    }
    //returns if player was in the lobby
    fn set_player_outside_lobby(&mut self, address: &SocketAddr, rejoinable: bool) -> bool {
        let Some(listener_client) = self.clients.get_mut(address) else {
            log!(error "Listener"; "{} {}", "Attempted set_player_outside_lobby with address that isn't in the map", address);
            return false;
        };
        listener_client.connection.send(ToClientPacket::ForcedOutsideLobby);
        let ListenerClientLocation::InLobby { ref mut room_code, ref mut lobby_client_id } = listener_client.location else {return false};
        if let Some(lobby) = self.lobbies.get_mut(room_code) {
            if rejoinable {
                lobby.remove_player_rejoinable(*lobby_client_id);
            }else{
                lobby.remove_player(*lobby_client_id)
            }
        }
        listener_client.location = ListenerClientLocation::OutsideLobby;
        true
    }
    
    pub fn create_client(&mut self, connection: &Connection) {
        let _ = self.delete_client(connection.get_address());
        self.clients.insert(*connection.get_address(), Client::new(connection.clone()));
    }
    pub fn delete_client(&mut self, address: &SocketAddr) -> Result<(), &'static str> {
        let Some(listener_client) = self.clients.remove(address) else {return Err("Player doesn't exist")};

        //This produces a warning in the logs because sometimes the player is already disconnected
        //This ToClientPacket is still useful in the *rare* case that the player is still connected when they're being forced to disconnect
        //A player can be forced to disconnect if a seperate connection is made with the same ip and port address
        listener_client.send(ToClientPacket::ForcedDisconnect);


        let ClientLocation::InLobby { room_code, lobby_client_id } = listener_client.location() else {return Ok(())};
        let Some(lobby) = self.lobbies.get_mut(&room_code) else {return Ok(())};

        lobby.remove_player_rejoinable(*lobby_client_id);

        Ok(())
    }


    fn valid_client(&self, addr: &SocketAddr)->Result<ClientReference,ValidateClientError>{
        let Some(client_ref) = ClientReference::new(addr, self) else {return Err(ValidateClientError::ClientDoesntExist)};
        if let Err(GetLobbyError::LobbyDoesntExist) = client.get_lobby(self) {return Err(ValidateClientError::InLobbyThatDoesntExist)};
        Ok(client_ref)
    }
}

enum ValidateClientError{
    ClientDoesntExist,
    InLobbyThatDoesntExist
}