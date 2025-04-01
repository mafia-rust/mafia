use std::collections::HashMap;

use crate::{log, packet::{LobbyPreviewData, RejectJoinReason, ToClientPacket, ToServerPacket}};

use super::{client::{ClientLocation, ClientReference}, RoomCode, WebsocketListener};

impl WebsocketListener{
    pub(super) fn handle_message(&mut self, client: ClientReference, packet: ToServerPacket) {

        match packet {
            ToServerPacket::Ping => {
                client.deref_mut(self).on_ping();
            },
            ToServerPacket::LobbyListRequest => {
                client.send(
                    self,
                    ToClientPacket::LobbyList{lobbies: self.lobbies()
                        .iter()
                        .map(|(room_code, lobby)|
                            (*room_code, LobbyPreviewData { 
                                name: lobby.name.clone(),
                                in_game: lobby.is_in_game(),
                                players: lobby.get_player_list() 
                            }
                        ))
                        .collect::<HashMap<RoomCode, LobbyPreviewData>>()
                    }
                );
            },
            ToServerPacket::ReJoin {room_code, player_id } => {
                self.set_client_in_lobby_reconnect(client, room_code, player_id);
            }
            ToServerPacket::Join{ room_code } => {
                self.set_client_in_lobby(&client, room_code);
            },
            ToServerPacket::Host => {
                let Some(room_code) = self.create_lobby() else {
                    client.deref(self).send(ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    return;
                };
                
                self.set_client_in_lobby(&client, room_code);

                log!(important "Lobby"; "Created {room_code}");
            },
            ToServerPacket::Leave => {
                self.set_client_outside_lobby(&client, false);
            },
            ToServerPacket::Kick { player_id: kicked_player_id } => {


                let Ok((lobby,room_code,host_id)) = client.get_lobby_mut(self) else {return};

                if !lobby.is_host(host_id) {return}

                
                let kicked_player = 
                    ClientReference::all_clients(self)
                    .find(|c|
                        *c.location(self) == ClientLocation::InLobby { room_code, lobby_client_id: kicked_player_id }
                    );

                if let Some(kicked_player) = kicked_player {

                    kicked_player.send(self, ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    self.set_client_outside_lobby(&kicked_player, false);
                    
                }else{
                    //Nobody is connected to that lobby with that id,
                    //Maybe they already left

                    let Ok((lobby,_,_)) = client.get_lobby_mut(self) else {return};

                    lobby.remove_player(kicked_player_id);
                }
                
            },
            _ => {
                let sender = &client.sender(self);
                let Ok((lobby, _, id)) = client.get_lobby_mut(self) else {return};

                lobby.on_client_message(sender, id, packet);
            }
        }
    }
}