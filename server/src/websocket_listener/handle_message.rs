use std::collections::HashMap;

use crate::{log, packet::{LobbyPreviewData, ToClientPacket, ToServerPacket}, websocket_connections::connection::Connection};

use super::{ClientLocation, RoomCode, WebsocketListener};

impl WebsocketListener{
    pub(super) fn handle_message(&mut self, connection: &Connection, packet: ToServerPacket) {

        match packet {
            ToServerPacket::Ping => {
                client.on_ping();
            },
            ToServerPacket::LobbyListRequest => {
                connection.send(ToClientPacket::LobbyList{lobbies: self.lobbies.iter()
                    .map(|(room_code, lobby)|
                        (*room_code, LobbyPreviewData { 
                            name: lobby.name.clone(),
                            in_game: lobby.is_in_game(),
                            players: lobby.get_player_list() 
                        }
                    ))
                    .collect::<HashMap<RoomCode, LobbyPreviewData>>()});
            },
            ToServerPacket::ReJoin {room_code, player_id } => {
                self.set_player_in_lobby_reconnect(connection, room_code, player_id);
            }
            ToServerPacket::Join{ room_code } => {
                self.set_player_in_lobby_initial_connect(connection, room_code);
            },
            ToServerPacket::Host => {
                let Some(room_code) = self.create_lobby() else {
                    connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return;
                };
                
                self.set_player_in_lobby_initial_connect(connection, room_code);

                log!(important "Lobby"; "Created {room_code}");
            },
            ToServerPacket::Leave => {
                self.set_player_outside_lobby(connection.get_address(), false);
            },
            ToServerPacket::Kick { player_id: kicked_player_id } => {
                let Some(host_location) = self.clients
                    .get(connection.get_address())
                    .map(|p|&p.location())
                else{
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from unconnected player!", connection.get_address());
                    return;
                };

                let ClientLocation::InLobby{room_code, lobby_client_id: host_id} = host_location else {
                    log!(error "Listener"; "{} {}", "Received lobby/game packet from player not in a lobby!", connection.get_address());
                    return;
                };

                if let Some(lobby) = self.lobbies.get_mut(room_code){
                    if !lobby.is_host(*host_id) {return Ok(());}

                    let kicked_player = self.clients
                        .iter()
                        .find(|(k,v)|*v.location() == ClientLocation::InLobby { room_code: *room_code, lobby_client_id: kicked_player_id })
                        .map(|(l,_)|l);

                    if let Some(kicked_player_address) = kicked_player {
                        if let Some(connection) = self.clients.get(&kicked_player_address).map(|p|p.connection.clone()) {
                            connection.send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                            self.set_player_outside_lobby(&kicked_player_address, false);
                        }
                    }else{
                        //Nobody is connected to that lobby with that id,
                        //Maybe they already left

                        if let Some(lobby) = self.lobbies.get_mut(room_code){
                            lobby.remove_player(kicked_player_id);
                        }
                    }
                }
            },
            _ => {
                let Ok((lobby, room_code, id)) = client.get_lobby(&self) else {return};

                lobby.on_client_message(&connection.get_sender(), *id, packet);    
                log!(error "listener.rs"; "Received packet from a client in a lobby that doesnt exist");
            }
        }
    }


    pub(super) fn check_client() {

    }
}