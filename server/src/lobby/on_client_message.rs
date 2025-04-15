use std::collections::VecDeque;

use crate::{game::{chat::{ChatMessage, ChatMessageVariant}, phase::PhaseType, player::{PlayerIndex, PlayerInitializeParameters}, spectator::{spectator_pointer::SpectatorIndex, SpectatorInitializeParameters}, Game, RejectStartReason}, log, packet::{ToClientPacket, ToServerPacket}, room::{game_client::{GameClient, GameClientLocation}, name_validation::{self, sanitize_server_name}, RemoveRoomClientResult, RoomClientID, RoomState}, strings::TidyableString, vec_map::VecMap, websocket_connections::connection::ClientSender};

use super::{lobby_client::{LobbyClient, LobbyClientType, Ready}, Lobby};


pub enum LobbyAction {
    StartGame(Game),
    Close,
    None
}

impl Lobby {
    pub fn on_client_message(&mut self, send: &ClientSender, room_client_id: RoomClientID, incoming_packet: ToServerPacket) -> LobbyAction {
        'packet_match: { match incoming_packet {
            ToServerPacket::SendLobbyMessage { text } => {
                let text = text.trim_newline().trim_whitespace().truncate(100);
                if text.is_empty() {break 'packet_match}
                
                let name = if let Some(
                    LobbyClient { client_type: LobbyClientType::Player { name }, .. }
                ) = self.clients.get(&room_client_id) {
                    name.clone()
                } else {
                    break 'packet_match
                };

                self.send_to_all(ToClientPacket::AddChatMessages { chat_messages: vec![
                    ChatMessage::new_non_private(
                        ChatMessageVariant::LobbyMessage { sender: name, text }, 
                        crate::game::chat::ChatGroup::All
                    )
                ]});
            }
            ToServerPacket::SetSpectator { spectator } => {
                let player_names = self.clients.values().filter_map(|p| {
                    if let LobbyClientType::Player { name } = p.client_type.clone() {
                        Some(name)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();

                let new_name = name_validation::sanitize_name("".to_string(), &player_names);

                if let Some(player) = self.clients.get_mut(&room_client_id){
                    match &player.client_type {
                        LobbyClientType::Spectator => {
                            if !spectator {
                                player.client_type = LobbyClientType::Player { name: new_name}
                            }
                        },
                        LobbyClientType::Player { .. } => {
                            if spectator {
                                player.client_type = LobbyClientType::Spectator;
                            }
                        },
                    }
                }

                self.set_rolelist_length();
                self.send_players();
                let role_list = self.settings.role_list.clone();
                self.send_to_all(ToClientPacket::RoleList { role_list } );
            }
            ToServerPacket::SetName{ name } => {
                self.set_player_name(room_client_id, name);
            },
            ToServerPacket::ReadyUp{ ready } => {
                if let Some(player) = self.clients.get_mut(&room_client_id){
                    if player.ready != Ready::Host {
                        player.ready = if ready { Ready::Ready } else { Ready::NotReady }
                    }
                }


                let mut ready = Vec::new();
                for client in self.clients.iter() {
                    if client.1.ready == Ready::Ready {
                        ready.push(*client.0);
                    }
                }
                self.send_to_all(ToClientPacket::PlayersReady { ready });
            },
            ToServerPacket::SetRoomName{ name } => {
                if let Some(client) = self.clients.get(&room_client_id) {
                    if !client.is_host() {break 'packet_match};
                }

                let name = sanitize_server_name(name);
                let name = if name.is_empty() {
                    name_validation::DEFAULT_SERVER_NAME.to_string()
                } else {
                    name
                };

                self.name.clone_from(&name);
                
                self.send_to_all(ToClientPacket::RoomName { name })
            },
            ToServerPacket::StartGame => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }
                
                let mut game_clients: VecMap<RoomClientID, GameClient> = VecMap::new();
                let mut game_player_params = Vec::new();
                let mut game_spectator_params = Vec::new();

                let mut next_player_index: PlayerIndex = 0;
                let mut next_spectator_index: SpectatorIndex = 0;

                for (room_client_id, lobby_client) in self.clients.clone() {
                    
                    game_clients.insert(room_client_id, 
                        if let LobbyClientType::Spectator = lobby_client.client_type {
                            GameClient {
                                client_location: GameClientLocation::Spectator(next_spectator_index),
                                host: lobby_client.is_host(),
                                last_message_times: VecDeque::new(),
                            }
                        } else {
                            GameClient {
                                client_location: GameClientLocation::Player(next_player_index),
                                host: lobby_client.is_host(),
                                last_message_times: VecDeque::new(),
                            }
                        }
                    );
                    
                    match lobby_client.client_type {
                        LobbyClientType::Player { ref name } => {
                            game_player_params.push(PlayerInitializeParameters{
                                host: lobby_client.is_host(),
                                connection: lobby_client.connection,
                                name: name.clone(),
                            });
                            if let Some(new_player_index) = next_player_index.checked_add(1) {
                                next_player_index = new_player_index;
                            } else {
                                send.send(ToClientPacket::RejectStart { reason: RejectStartReason::TooManyClients });
                                break 'packet_match;
                            }
                        },
                        LobbyClientType::Spectator => {
                            game_spectator_params.push(SpectatorInitializeParameters{
                                host: lobby_client.is_host(),
                                connection: lobby_client.connection,
                            });
                            if let Some(new_spectator_index) = next_spectator_index.checked_add(1) {
                                next_spectator_index = new_spectator_index;
                            } else {
                                send.send(ToClientPacket::RejectStart { reason: RejectStartReason::TooManyClients });
                                break 'packet_match;
                            }
                        }
                    }
                }

                let game = match Game::new(self.name.clone(), self.settings.clone(), game_clients, game_player_params, game_spectator_params){
                    Ok(game) => game,
                    Err(err) => {
                        send.send(ToClientPacket::RejectStart { reason: err });
                        log!(info "Lobby"; "Failed to start game: {:?}", err);
                        break 'packet_match
                    }
                };
                        
                self.send_to_all(ToClientPacket::RoomName { name: self.name.clone() });

                return LobbyAction::StartGame(game);
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }

                match phase {
                    PhaseType::Briefing => { self.settings.phase_times.briefing = time; }
                    PhaseType::Obituary => { self.settings.phase_times.obituary = time; }
                    PhaseType::Discussion => { self.settings.phase_times.discussion = time; }
                    PhaseType::FinalWords => { self.settings.phase_times.final_words = time; }
                    PhaseType::Dusk => { self.settings.phase_times.dusk = time; }
                    PhaseType::Judgement => { self.settings.phase_times.judgement = time; }
                    PhaseType::Night => { self.settings.phase_times.night = time; }
                    PhaseType::Testimony => { self.settings.phase_times.testimony = time; }
                    PhaseType::Nomination => { self.settings.phase_times.nomination = time; }
                    PhaseType::Recess => { }
                };
                
                self.send_to_all(ToClientPacket::PhaseTime { phase, time });
            },
            ToServerPacket::SetPhaseTimes { phase_time_settings } => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }

                self.settings.phase_times = phase_time_settings.clone();

                self.send_to_all(ToClientPacket::PhaseTimes { phase_time_settings });
            }
            ToServerPacket::SetRoleList { role_list } => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }

                self.settings.role_list = role_list;
                self.set_rolelist_length();
                
                let role_list = self.settings.role_list.clone();

                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetRoleOutline { index, role_outline } => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }

                if self.settings.role_list.0.len() <= index as usize {break 'packet_match}
                let Some(unset_outline) = self.settings.role_list.0.get_mut(index as usize) else {break 'packet_match};
                *unset_outline = role_outline.clone();
                
                self.send_to_all(ToClientPacket::RoleOutline { index, role_outline });
            }
            ToServerPacket::SimplifyRoleList => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.is_host() {break 'packet_match}
                }

                self.settings.role_list.simplify();
                let role_list = self.settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetEnabledRoles { roles } => {
                self.settings.enabled_roles = roles.into_iter().collect();
                let roles = self.settings.enabled_roles.clone().into_iter().collect();
                self.send_to_all(ToClientPacket::EnabledRoles { roles });
            }
            ToServerPacket::SetEnabledModifiers {modifiers } => {
                self.settings.enabled_modifiers = modifiers.into_iter().collect();
                let modifiers = self.settings.enabled_modifiers.clone().into_iter().collect();
                self.send_to_all(ToClientPacket::EnabledModifiers { modifiers });
            }
            ToServerPacket::Leave => {
                if let RemoveRoomClientResult::RoomShouldClose = self.remove_client(room_client_id) {
                    return LobbyAction::Close;
                }
            }
            ToServerPacket::HostForceSetPlayerName { id, name } => {
                if let Some(player) = self.clients.get(&room_client_id) {
                    if !player.is_host() { break 'packet_match }
                }
                self.set_player_name(id, name);
            }
            ToServerPacket::SetPlayerHost { player_id } => {
                if let Some(player) = self.clients.get(&room_client_id) {
                    if !player.is_host() { break 'packet_match }
                }
                if let Some(player) = self.clients.get_mut(&player_id) {
                    player.set_host();
                }
                self.send_players();
            }
            ToServerPacket::RelinquishHost => {
                if let Some(player) = self.clients.get_mut(&room_client_id) {
                    if !player.is_host() { break 'packet_match }
                    player.relinquish_host();
                }
                self.ensure_host_exists(Some(room_client_id));
                self.send_players();
            }
            _ => {
                log!(error "Lobby"; "{} {:?}", "ToServerPacket not implemented for lobby was sent during lobby: ", incoming_packet);
            }
        } }

        LobbyAction::None
    }
}