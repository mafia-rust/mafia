use std::collections::HashMap;

use crate::{game::on_client_message::GameAction, lobby::{on_client_message::RoomAction, on_lobby_message::LobbyAction, RemoveClientData, RoomState}, log, packet::{LobbyPreviewData, RejectJoinReason, ToClientPacket, ToServerPacket}};

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
                        .map(|(room_code, room)| (*room_code, room.get_preview_data()))
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


                let Ok((lobby,room_code,host_id)) = client.get_room_mut(self) else {return};

                if !lobby.is_host(host_id) {return}

                
                let kicked_player = 
                    ClientReference::all_clients(self)
                    .find(|c|
                        *c.location(self) == ClientLocation::InRoom { room_code, room_client_id: kicked_player_id }
                    );

                if let Some(kicked_player) = kicked_player {

                    kicked_player.send(self, ToClientPacket::RejectJoin { reason: RejectJoinReason::ServerBusy });
                    self.set_client_outside_lobby(&kicked_player, false);
                    
                }else{
                    //Nobody is connected to that lobby with that id,
                    //Maybe they already left

                    let Ok((lobby,_,_)) = client.get_room_mut(self) else {return};

                    let RemoveClientData { close_room } = lobby.remove_client(kicked_player_id);

                    if close_room {
                        self.delete_lobby(room_code);
                    }
                }
                
            },
            _ => {
                let sender = &client.sender(self);
                let Ok((room, room_code, id)) = client.get_room_mut(self) else {return};

                match room.on_client_message(sender, id, packet) {
                    RoomAction::LobbyAction(LobbyAction::StartGame(game)) => {
                        log!(info "Lobby"; "Game started with room code {}", room_code);

                        *room = RoomState::Game(game);
                    },
                    RoomAction::GameAction(GameAction::BackToLobby(lobby)) => {
                        *room = RoomState::Lobby(lobby);
                    },
                    RoomAction::GameAction(GameAction::Close) |
                    RoomAction::LobbyAction(LobbyAction::Close) => {
                        self.delete_lobby(room_code);
                    },
                    _ => {}
                }
                
            }
        }
    }
}