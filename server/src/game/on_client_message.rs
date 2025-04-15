use crate::{lobby::{lobby_client::LobbyClient, Lobby}, log, packet::{ToClientPacket, ToServerPacket}, room::{RemoveRoomClientResult, RoomClientID, RoomState}, strings::TidyableString, vec_map::VecMap, websocket_connections::connection::ClientSender};

use super::{
    chat::{ChatGroup, ChatMessageVariant, MessageSender}, event::{on_fast_forward::OnFastForward, on_game_ending::OnGameEnding, on_whisper::OnWhisper, Event}, game_client::GameClientLocation, game_conclusion::GameConclusion, phase::PhaseType, player::PlayerReference, role::{
        Role, RoleState
    }, spectator::spectator_pointer::SpectatorPointer, Game
};



pub enum GameAction {
    BackToLobby(Lobby),
    Close,
    None
}

impl Game {
    #[expect(clippy::match_single_binding, unused, reason="Surely spectators will do something in the future")]
    pub fn on_spectator_message(&mut self, sender_ref: SpectatorPointer, incoming_packet: ToServerPacket){
        match incoming_packet {
            _ => {}
        }
    }
    
    pub fn on_client_message(&mut self, _: &ClientSender, room_client_id: RoomClientID, incoming_packet: ToServerPacket) -> GameAction {
        if let Some(client) = self.clients.get(&room_client_id) {
            match client.client_location {
                GameClientLocation::Player(player) => {
                    self.on_player_message(room_client_id, player, incoming_packet)
                }
                GameClientLocation::Spectator(spectator) => {
                    self.on_spectator_message(spectator, incoming_packet);
                    GameAction::None
                }
            }
        } else {
            log!(error "Game"; "Received message from invalid client id: {}", room_client_id);
            GameAction::None
        }
    }

    pub fn on_player_message(&mut self, room_client_id: RoomClientID, sender_player_ref: PlayerReference, incoming_packet: ToServerPacket) -> GameAction {
        'packet_match: {match incoming_packet {
            ToServerPacket::SetName{ name } => {
                self.set_player_name(sender_player_ref, name);
            },
            ToServerPacket::Leave => {
                if let RemoveRoomClientResult::RoomShouldClose = self.remove_client(room_client_id) {
                    return GameAction::Close;
                }
            },
            ToServerPacket::HostForceBackToLobby => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }

                self.settings.role_list.simplify();
                let role_list = self.settings.role_list.clone();
                
                self.send_to_all(ToClientPacket::RoleList { role_list });

                let mut new_clients = VecMap::new();
                for (room_client_id, game_client) in self.clients.clone() {
                    new_clients.insert(room_client_id, LobbyClient::new_from_game_client(self, game_client));
                }

                self.send_to_all(ToClientPacket::BackToLobby);

                let lobby = Lobby::new_from_game(self.room_name.clone(), self.settings.clone(), new_clients);

                return GameAction::BackToLobby(lobby);
            }
            ToServerPacket::HostForceEndGame => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }

                let conclusion = GameConclusion::get_premature_conclusion(self);

                OnGameEnding::new(conclusion).invoke(self);
            }
            ToServerPacket::HostForceSkipPhase => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }
                
                OnFastForward::invoke(self);
            }
            ToServerPacket::HostDataRequest => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }

                self.resend_host_data(sender_player_ref.connection(self));
            }
            ToServerPacket::HostForceSetPlayerName { id, name } => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }
                if let Some(player) = self.clients.get(&id) {
                    if let GameClientLocation::Player(player) = player.client_location {
                        self.set_player_name(player, name);
                    }
                }
            }
            ToServerPacket::SetPlayerHost { player_id } => {
                if let Some(player) = self.clients.get(&room_client_id){
                    if !player.host {break 'packet_match}
                }
                if let Some(player) = self.clients.get_mut(&player_id) {
                    player.set_host();
                }
                self.send_players();
                self.resend_host_data_to_all_hosts();
            }
            ToServerPacket::RelinquishHost => {
                if let Some(player) = self.clients.get_mut(&room_client_id){
                    if !player.host {break 'packet_match}
                    player.relinquish_host();
                }
                self.ensure_host_exists(Some(room_client_id));
                self.send_players();
                self.resend_host_data_to_all_hosts();
            }
            ToServerPacket::Judgement { verdict } => {
                if self.current_phase().phase() != PhaseType::Judgement {break 'packet_match;}
                
                sender_player_ref.set_verdict(self, verdict);
            },
            ToServerPacket::SendChatMessage { text, block } => {
                if text.replace(['\n', '\r'], "").trim().is_empty() {
                    break 'packet_match;
                }
                
                for chat_group in sender_player_ref.get_current_send_chat_groups(self){
                    let message_sender = match chat_group {
                        ChatGroup::Jail => {
                            if sender_player_ref.role(self) == Role::Jailor {
                                Some(MessageSender::Jailor)
                            }else{None}
                        },
                        ChatGroup::Kidnapped => {
                            if sender_player_ref.role(self) == Role::Kidnapper {
                                Some(MessageSender::Jailor)
                            }else{None}
                        },
                        ChatGroup::Dead => {
                            if sender_player_ref.alive(self) {
                                Some(MessageSender::LivingToDead{ player: sender_player_ref.index() })
                            }else{None}
                        },
                        ChatGroup::Interview => {
                            if sender_player_ref.role(self) == Role::Reporter {
                                Some(MessageSender::Reporter)
                            }else{None}
                        },
                        _ => {None}
                    };

                    let message_sender = message_sender.unwrap_or(MessageSender::Player { player: sender_player_ref.index() });


                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        ChatMessageVariant::Normal{
                            message_sender,
                            text: text.trim_newline().trim_whitespace().truncate(600).truncate_lines(35), 
                            block
                        }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {
                let whisperee_ref = match PlayerReference::new(self, whispered_to_player_index) {
                    Ok(receiver_ref) => receiver_ref,
                    Err(_) => {
                        sender_player_ref.add_private_chat_message(self, ChatMessageVariant::InvalidWhisper);
                        break 'packet_match;
                    }
                };

                OnWhisper::new(sender_player_ref, whisperee_ref, text).invoke(self);
            },
            ToServerPacket::SaveWill { will } => {
                sender_player_ref.set_will(self, will);
            },
            ToServerPacket::SaveNotes { notes } => {
                sender_player_ref.set_notes(self, notes);
            },
            ToServerPacket::SaveCrossedOutOutlines { crossed_out_outlines } => {
                sender_player_ref.set_crossed_out_outlines(self, crossed_out_outlines);
            },
            ToServerPacket::SaveDeathNote { death_note } => {
                sender_player_ref.set_death_note(self, death_note);
            },
            ToServerPacket::AbilityInput { ability_input } => 
                ability_input.on_client_message(self, sender_player_ref),
            ToServerPacket::SetDoomsayerGuess { guesses } => {
                if let RoleState::Doomsayer(mut doomsayer) = sender_player_ref.role_state(self).clone(){
                    doomsayer.guesses = guesses;
                    sender_player_ref.set_role_state(self, RoleState::Doomsayer(doomsayer));
                }
            },
            ToServerPacket::SetConsortOptions { 
                roleblock, 
                you_were_roleblocked_message, 
                you_survived_attack_message, 
                you_were_protected_message, 
                you_were_transported_message, 
                you_were_possessed_message, 
                your_target_was_jailed_message 
            } => {
                if let RoleState::Hypnotist(mut hypnotist) = sender_player_ref.role_state(self).clone(){
                    hypnotist.roleblock = roleblock;

                    hypnotist.you_were_roleblocked_message = you_were_roleblocked_message;
                    hypnotist.you_survived_attack_message = you_survived_attack_message;
                    hypnotist.you_were_protected_message = you_were_protected_message;
                    hypnotist.you_were_transported_message = you_were_transported_message;
                    hypnotist.you_were_possessed_message = you_were_possessed_message;
                    hypnotist.your_target_was_jailed_message = your_target_was_jailed_message;

                    //There must be at least one message enabled, so if none are, enable roleblocked message
                    hypnotist.ensure_at_least_one_message();

                    sender_player_ref.set_role_state(self, RoleState::Hypnotist(hypnotist));
                }
            },
            ToServerPacket::VoteFastForwardPhase { fast_forward } => {
                sender_player_ref.set_fast_forward_vote(self, fast_forward);
            },
            _ => {
                log!(fatal "Game"; "Unimplemented ToServerPacket: {incoming_packet:?}");
                unreachable!();
            }
        }}
        
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_repeating_data(self)
        }
        for spectator_ref in SpectatorPointer::all_spectators(self){
            spectator_ref.send_repeating_data(self)
        }

        GameAction::None
    }
}