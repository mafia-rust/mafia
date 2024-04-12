use std::{collections::{HashMap, HashSet, VecDeque}, time::{Duration, Instant}};

use crate::{game::{phase::PhaseType, player::{PlayerIndex, PlayerInitializeParameters}, spectator::{spectator_pointer::SpectatorIndex, SpectatorInitializeParameters}, Game}, lobby::game_client::{GameClient, GameClientLocation}, log, packet::{ToClientPacket, ToServerPacket}, websocket_connections::connection::ClientSender};

use super::{lobby_client::{LobbyClientID, LobbyClientType}, name_validation::{self, sanitize_server_name}, Lobby, LobbyState};

pub const MESSAGE_PER_SECOND_LIMIT: u64 = 2;
pub const MESSAGE_PER_SECOND_LIMIT_TIME: Duration = Duration::from_secs(2);

impl Lobby {
    pub fn on_client_message(&mut self, send: &ClientSender, lobby_client_id: LobbyClientID, incoming_packet: ToServerPacket){

        //RATE LIMITER
        match incoming_packet {
            ToServerPacket::Vote { .. } |
            ToServerPacket::Judgement { .. } |
            ToServerPacket::Target { .. } |
            ToServerPacket::DayTarget { .. } |
            ToServerPacket::SendMessage { .. } |
            ToServerPacket::SendWhisper { .. } => {
                let LobbyState::Game { clients, .. } = &mut self.lobby_state else {
                    return;
                };

                let Some(game_player) = clients.get_mut(&lobby_client_id) else {
                    log!(error "LobbyState::Game"; "{} {:?}", "Message recieved from player not in game", incoming_packet);
                    return;
                };

                let now = Instant::now();
                while let Some(time) = game_player.last_message_times.front() {
                    if now.duration_since(*time) > MESSAGE_PER_SECOND_LIMIT_TIME {
                        game_player.last_message_times.pop_front();
                    } else {
                        break;
                    }
                }
                if game_player.last_message_times.len() >= (MESSAGE_PER_SECOND_LIMIT_TIME.as_secs() * MESSAGE_PER_SECOND_LIMIT) as usize {
                    send.send(ToClientPacket::RateLimitExceeded);
                    return;
                }
                game_player.last_message_times.push_back(now);
                
            },
            _ => {}
        }



        match incoming_packet {
            ToServerPacket::SetSpectator { spectator } => {
                let LobbyState::Lobby { clients, settings } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", lobby_client_id);
                    return
                };
                
                let new_name = name_validation::sanitize_name("".to_string(), &clients);
                if let Some(player) = clients.get_mut(&lobby_client_id){
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

                Lobby::set_rolelist_length(settings, clients);
                Self::send_players_lobby(clients);
                let role_list = settings.role_list.clone();
                self.send_to_all(ToClientPacket::RoleList { role_list: role_list } );
            }
            ToServerPacket::SetName{ name } => {
                let LobbyState::Lobby { clients, .. } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetName can not be used outside of LobbyState::Lobby", lobby_client_id);
                    return
                };

                let mut other_players = clients.clone();
                other_players.remove(&lobby_client_id);
                
                let new_name: String = name_validation::sanitize_name(name, &other_players);
                if let Some(player) = clients.get_mut(&lobby_client_id){
                    if let LobbyClientType::Player { name } = &mut player.client_type {
                        *name = new_name;
                    }
                }

                Self::send_players_lobby(clients);
            },
            ToServerPacket::SetLobbyName{ name } => {
                let LobbyState::Lobby { .. } = self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::SetLobbyName can not be used outside of LobbyState::Lobby", lobby_client_id);
                    return
                };

                if !self.is_host(lobby_client_id) {return};

                let name = sanitize_server_name(name);
                let name = if name.is_empty() {
                    self.name = name_validation::DEFAULT_SERVER_NAME.to_string();
                    self.name.clone()
                } else {
                    self.name = name.clone();
                    self.name.clone()
                };
                
                self.send_to_all(ToClientPacket::LobbyName { name })
            },
            ToServerPacket::StartGame => {
                let LobbyState::Lobby { settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "ToServerPacket::StartGame can not be used outside of LobbyState::Lobby", lobby_client_id);
                    return
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                settings.role_list.simplify();
                let role_list = settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });

                let mut game_clients: HashMap<LobbyClientID, GameClient> = HashMap::new();
                let mut game_player_params = Vec::new();
                let mut game_spectator_params = Vec::new();


                let LobbyState::Lobby { settings, clients} = &mut self.lobby_state else {
                    unreachable!("LobbyState::Lobby was checked to be to LobbyState::Lobby in the previous line")
                };

                let mut next_player_index: PlayerIndex = 0;
                let mut next_spectator_index: SpectatorIndex = 0;

                for (lobby_client_id, lobby_client) in clients.clone().into_iter() {
                    
                    game_clients.insert(lobby_client_id, 
                        if let LobbyClientType::Spectator = lobby_client.client_type {
                            GameClient {
                                client_location: GameClientLocation::Spectator(next_spectator_index),
                                host: lobby_client.host,
                                last_message_times: VecDeque::new(),
                            }
                        } else {
                            GameClient {
                                client_location: GameClientLocation::Player(next_player_index),
                                host: lobby_client.host,
                                last_message_times: VecDeque::new(),
                            }
                        }
                    );
                    
                    match lobby_client.client_type {
                        LobbyClientType::Player { name } => {
                            game_player_params.push(PlayerInitializeParameters{
                                connection: lobby_client.connection,
                                name,
                                host: lobby_client.host,
                            });
                            next_player_index += 1;
                        },
                        LobbyClientType::Spectator => {
                            game_spectator_params.push(SpectatorInitializeParameters{
                                connection: lobby_client.connection,
                                host: lobby_client.host,
                            });
                            next_spectator_index += 1;
                        }
                    }
                }

                let game = match Game::new(settings.clone(), game_player_params, game_spectator_params){
                    Ok(game) => game,
                    Err(err) => {
                        send.send(ToClientPacket::RejectStart { reason: err });
                        log!(info "Lobby"; "Failed to start game: {:?}", err);
                        return
                    }
                };
                
                log!(info "Lobby"; "Game started with room code {}", self.room_code);

                self.lobby_state = LobbyState::Game{
                    game,
                    clients: game_clients,
                };
                let LobbyState::Game { game, clients: _player } = &mut self.lobby_state else {
                    unreachable!("LobbyState::Game was set to be to LobbyState::Game in the previous line");
                };

                Lobby::send_players_game(game);
            },
            ToServerPacket::SetPhaseTime{phase, time} => {
                let LobbyState::Lobby{ settings, clients  } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                match phase {
                    PhaseType::Briefing => { settings.phase_times.briefing = time; }
                    PhaseType::Obituary => { settings.phase_times.obituary = time; }
                    PhaseType::Discussion => { settings.phase_times.discussion = time; }
                    PhaseType::FinalWords => { settings.phase_times.final_words = time; }
                    PhaseType::Dusk => { settings.phase_times.dusk = time; }
                    PhaseType::Judgement => { settings.phase_times.judgement = time; }
                    PhaseType::Night => { settings.phase_times.night = time; }
                    PhaseType::Testimony => { settings.phase_times.testimony = time; }
                    PhaseType::Nomination => { settings.phase_times.nomination = time; }
                };
                
                self.send_to_all(ToClientPacket::PhaseTime { phase, time });
            },
            ToServerPacket::SetPhaseTimes { phase_time_settings } => {
                let LobbyState::Lobby{ settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Attempted to change phase time outside of the lobby menu!", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                settings.phase_times = phase_time_settings.clone();

                self.send_to_all(ToClientPacket::PhaseTimes { phase_time_settings });
            }
            ToServerPacket::SetRoleList { role_list } => {
                let LobbyState::Lobby{ settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                settings.role_list = role_list.clone();
                Lobby::set_rolelist_length(settings, clients);
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetRoleOutline { index, role_outline } => {
                let LobbyState::Lobby{ settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                if settings.role_list.0.len() <= index as usize {return}
                let Some(unset_outline) = settings.role_list.0.get_mut(index as usize) else {return};
                *unset_outline = role_outline.clone();
                
                self.send_to_all(ToClientPacket::RoleOutline { index, role_outline });
            }
            ToServerPacket::SimplifyRoleList => {
                let LobbyState::Lobby{ settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return}
                }

                settings.role_list.simplify();
                let role_list = settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });
            }
            ToServerPacket::SetExcludedRoles {mut roles } => {
                let LobbyState::Lobby{ settings, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {}", "Can't modify game settings outside of the lobby menu", lobby_client_id);
                    return;
                };
                if let Some(player) = clients.get(&lobby_client_id){
                    if !player.host {return;}
                }


                let roles = roles.drain(..).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
                settings.excluded_roles = roles.clone();
                self.send_to_all(ToClientPacket::ExcludedRoles { roles });
            }
            ToServerPacket::Leave => {
                self.remove_player(lobby_client_id);
            }
            _ => {
                let LobbyState::Game { game, clients } = &mut self.lobby_state else {
                    log!(error "Lobby"; "{} {:?}", "ToServerPacket not implemented for lobby was sent during lobby: ", incoming_packet);
                    return;
                };
                
                match clients[&lobby_client_id].client_location {
                    GameClientLocation::Player(player_index) => {
                        game.on_client_message(player_index, incoming_packet)
                    }
                    GameClientLocation::Spectator(spectator_index) => {
                        game.on_spectator_message(spectator_index, incoming_packet)
                    }
                }
            }
        }
    }
}