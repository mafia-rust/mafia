mod event;
mod client;
mod handle_message;

pub type RoomCode = usize;


use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};

use crate::{lobby::{lobby_client::LobbyClientID, Lobby}, packet::{RejectJoinReason, ToClientPacket}, websocket_connections::connection::Connection};

use self::client::{Client, ClientLocation, ClientReference, GetLobbyError};
use rand::random;



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
    fn clients(&self) -> &HashMap<SocketAddr, Client> {
        &self.clients
    }
    fn lobbies(&self) -> &HashMap<RoomCode, Lobby> {
        &self.lobbies
    }
    fn lobbies_mut(&mut self) -> &mut HashMap<RoomCode, Lobby> {
        &mut self.lobbies
    }
    fn get_client<'a>(&'a self, address: &SocketAddr) -> Option<&'a Client> {
        self.clients.get(address)
    }
    fn get_client_mut<'a>(&'a mut self, address: &SocketAddr) -> Option<&'a mut Client> {
        self.clients.get_mut(address)
    }
    pub(super) fn get_lobby<'a>(&'a self, room_code: &RoomCode) -> Option<&'a Lobby> {
        self.lobbies.get(room_code)
    }
    pub(super) fn get_lobby_mut<'a>(&'a mut self, room_code: &RoomCode) -> Option<&'a mut Lobby> {
        self.lobbies.get_mut(room_code)
    }


    pub(super) fn create_client(&mut self, connection: &Connection) {

        if let Some(client_already_exists) = ClientReference::new(connection.address(), self){
            self.delete_client(&client_already_exists);
        }

        self.clients.insert(*connection.address(), Client::new(connection.clone()));
    }
    fn delete_client(&mut self, client: &ClientReference) {
        let Some(client) = self.clients.remove(&client.address(self).clone()) else {return};

        //This produces a warning in the logs because sometimes the player is already disconnected
        //This ToClientPacket is still useful in the *rare* case that the player is still connected when they're being forced to disconnect
        //A player can be forced to disconnect if a seperate connection is made with the same ip and port address
        client.send(ToClientPacket::ForcedDisconnect);


        let ClientLocation::InLobby { room_code, lobby_client_id } = client.location() else {return};
        let Some(lobby) = self.lobbies.get_mut(room_code) else {return};

        lobby.remove_player_rejoinable(*lobby_client_id);
    }


    fn set_client_in_lobby(&mut self, client: &ClientReference, room_code: RoomCode){

        let sender = &client.sender(self).clone();
        let Some(lobby) = self.get_lobby_mut(&room_code) else {
            client.send(self, ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return
        };
        let Ok(lobby_client_id) = lobby.join_player(sender) else {return};

        client.set_location(self, ClientLocation::InLobby { room_code, lobby_client_id });
    }
    fn set_client_in_lobby_reconnect(&mut self, client: ClientReference, room_code: RoomCode, lobby_client_id: LobbyClientID){

        let sender = &client.sender(self).clone();
        let Some(lobby) = self.get_lobby_mut(&room_code) else {
            client.send(self, ToClientPacket::RejectJoin { reason: RejectJoinReason::RoomDoesntExist });
            return
        };
        let Ok(()) = lobby.rejoin_player(sender, lobby_client_id) else {return};

        client.set_location(self, ClientLocation::InLobby { room_code, lobby_client_id });
    }
    fn set_client_outside_lobby(&mut self, client: &ClientReference, rejoinable: bool) {
        client.send(self, ToClientPacket::ForcedOutsideLobby);
        
        if let Ok((lobby, _, id)) = client.get_lobby_mut(self) {
            if rejoinable {
                lobby.remove_player_rejoinable(id);
            }else{
                lobby.remove_player(id)
            }
        }

        client.set_location(self, ClientLocation::OutsideLobby);
    }



    fn generate_roomcode(&self)->Option<RoomCode>{
        ((random::<u16>() as usize)..usize::MAX).find(
            |code| !self.lobbies.contains_key(code)
        )
    }
    pub(super) fn create_lobby(&mut self) -> Option<RoomCode>{
        let room_code = self.generate_roomcode()?;

        let lobby = Lobby::new(room_code);
        self.lobbies.insert(room_code, lobby);
        Some(room_code)
    }
    pub(super) fn delete_lobby(&mut self, room_code: RoomCode){

        for client in ClientReference::all_clients(self){
            if client.in_room(self, room_code) {
                self.set_client_outside_lobby(&client, false);
            }
        }

        self.lobbies.remove(&room_code);
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


    fn validate_client(&self, addr: &SocketAddr)->Result<ClientReference,ValidateClientError>{
        let Some(client) = ClientReference::new(addr, self) else {return Err(ValidateClientError::ClientDoesntExist)};
        if let Err(GetLobbyError::LobbyDoesntExist) = client.get_lobby(self) {return Err(ValidateClientError::InLobbyThatDoesntExist)};
        Ok(client)
    }
    
}


pub(super) enum ValidateClientError{
    ClientDoesntExist,
    InLobbyThatDoesntExist
}
