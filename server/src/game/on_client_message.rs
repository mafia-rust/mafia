use crate::{packet::ToServerPacket, strings::TidyableString, log};

use super::{
    chat::{ChatGroup, ChatMessageVariant, MessageSender},
    phase::{PhaseState, PhaseType},
    player::{PlayerIndex, PlayerReference},
    role::{Role, RoleState},
    Game
};




impl Game {
    pub fn on_client_message(&mut self, sender_player_index: PlayerIndex, incoming_packet: ToServerPacket){

        let sender_player_ref = match PlayerReference::new(self, sender_player_index){
            Ok(sender_player_ref) => sender_player_ref,
            Err(_) => {
                log!(error "Game"; "Received message from invalid player index: {}", sender_player_index);
                return;
            }
        };

        'packet_match: {match incoming_packet {
            ToServerPacket::Vote { player_index: player_voted_index } => {
                let &PhaseState::Nomination { .. } = self.current_phase() else {break 'packet_match};

                let player_voted_ref = match PlayerReference::index_option_to_ref(self, &player_voted_index){
                    Ok(player_voted_ref) => player_voted_ref,
                    Err(_) => break 'packet_match,
                };

                sender_player_ref.set_chosen_vote(self, player_voted_ref, true);

                self.count_votes_and_start_trial();
            },
            ToServerPacket::Judgement { verdict } => {
                if self.current_phase().phase() != PhaseType::Judgement {break 'packet_match;}
                
                sender_player_ref.set_verdict(self, verdict);
            },
            ToServerPacket::Target { player_index_list }=>{
                if self.current_phase().phase() != PhaseType::Night {break 'packet_match;}

                let target_ref_list = match PlayerReference::index_vec_to_ref(self, &player_index_list){
                    Ok(target_ref_list) => target_ref_list,
                    Err(_) => {
                        break 'packet_match;
                    },
                };
                sender_player_ref.set_chosen_targets(self, target_ref_list.clone());
                
                let mut target_message_sent = false;
                for chat_group in sender_player_ref.get_current_send_chat_groups(self){
                    match chat_group {
                        ChatGroup::All | ChatGroup::Interview | ChatGroup::Dead => {},
                        ChatGroup::Mafia | ChatGroup::Cult => {
                            self.add_message_to_chat_group( chat_group,
                                ChatMessageVariant::Targeted { 
                                    targeter: sender_player_ref.index(), 
                                    targets: PlayerReference::ref_vec_to_index(&target_ref_list)
                                }
                            );
                            target_message_sent = true;
                        },
                        ChatGroup::Jail => {
                            if sender_player_ref.role(self) == Role::Jailor {
                                self.add_message_to_chat_group(chat_group,
                                    ChatMessageVariant::JailorDecideExecute {
                                        target: target_ref_list.first().map(|p|p.index())
                                    }
                                );
                                target_message_sent = true;
                            }
                        },
                    }
                }
                
                
                if !target_message_sent{
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::Targeted { 
                        targeter: sender_player_ref.index(), 
                        targets: PlayerReference::ref_vec_to_index(&target_ref_list)
                    });
                }
            },
            ToServerPacket::DayTarget { player_index } => {               
                let target_ref = match PlayerReference::new(self, player_index){
                    Ok(target_ref) => target_ref,
                    Err(_) => break 'packet_match,
                };
                if sender_player_ref.can_day_target(self, target_ref){
                    sender_player_ref.do_day_action(self, target_ref);
                }
            },
            ToServerPacket::SendMessage { text } => {

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
                        ChatGroup::Dead => {
                            if sender_player_ref.alive(self) {
                                Some(MessageSender::LivingToDead{ player: sender_player_index })
                            }else{None}
                        },
                        ChatGroup::Interview => {
                            if sender_player_ref.role(self) == Role::Journalist {
                                Some(MessageSender::Journalist)
                            }else{None}
                        },
                        _ => {None}
                    };

                    let message_sender = message_sender.unwrap_or(MessageSender::Player { player: sender_player_index });


                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        ChatMessageVariant::Normal{
                            message_sender,
                            text: text.trim_newline().trim_whitespace().truncate(400).truncate_lines(20), 
                        }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {

                let whisperee_ref = match PlayerReference::new(self, whispered_to_player_index){
                    Ok(whisperee_ref) => whisperee_ref,
                    Err(_) => break 'packet_match,
                };

                if !self.current_phase().is_day() || 
                    whisperee_ref.alive(self) != sender_player_ref.alive(self) ||
                    whisperee_ref == sender_player_ref || 
                    !sender_player_ref.get_current_send_chat_groups(self).contains(&ChatGroup::All) ||
                    text.replace(['\n', '\r'], "").trim().is_empty()
                {
                    break 'packet_match;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::BroadcastWhisper { whisperer: sender_player_index, whisperee: whispered_to_player_index });
                let message = ChatMessageVariant::Whisper { 
                    from_player_index: sender_player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                whisperee_ref.add_private_chat_message(self, message.clone());
                sender_player_ref.add_private_chat_message(self, message.clone());

                for player in PlayerReference::all_players(self){
                    if player.role(self) == Role::Consigliere {
                        whisperee_ref.add_private_chat_message(self, message.clone());
                    }
                }
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
            ToServerPacket::SetDoomsayerGuess { guesses } => {
                if let RoleState::Doomsayer(mut doomsayer) = sender_player_ref.role_state(self).clone(){
                    doomsayer.guesses = guesses;
                    sender_player_ref.set_role_state(self, RoleState::Doomsayer(doomsayer));
                }
            }
            ToServerPacket::SetAmnesiacRoleOutline { role_outline } => {
                if let RoleState::Amnesiac(mut amnesiac) = sender_player_ref.role_state(self).clone(){
                    amnesiac.role_outline = role_outline;
                    sender_player_ref.set_role_state(self, RoleState::Amnesiac(amnesiac));
                }
            }
            ToServerPacket::SetJournalistJournal { journal } => {
                if let RoleState::Journalist(mut journalist) = sender_player_ref.role_state(self).clone(){
                    journalist.journal = journal;
                    sender_player_ref.set_role_state(self, RoleState::Journalist(journalist));
                }
            }
            ToServerPacket::SetJournalistJournalPublic { public } => {
                if let RoleState::Journalist(mut journalist) = sender_player_ref.role_state(self).clone(){
                    journalist.public = public;
                    sender_player_ref.set_role_state(self, RoleState::Journalist(journalist));
                }
            }
            ToServerPacket::SetConsortOptions { 
                roleblock, 
                you_were_roleblocked_message, 
                you_survived_attack_message, 
                you_were_protected_message, 
                you_were_transported_message, 
                you_were_possessed_message, 
                your_target_was_jailed_message 
            } => {
                if let RoleState::Consort(mut consort) = sender_player_ref.role_state(self).clone(){
                    consort.roleblock = roleblock;

                    consort.you_were_roleblocked_message = you_were_roleblocked_message;
                    consort.you_survived_attack_message = you_survived_attack_message;
                    consort.you_were_protected_message = you_were_protected_message;
                    consort.you_were_transported_message = you_were_transported_message;
                    consort.you_were_possessed_message = you_were_possessed_message;
                    consort.your_target_was_jailed_message = your_target_was_jailed_message;

                    //There must be at least one message enabled, so if none are, enable roleblocked message
                    consort.ensure_at_least_one_message();

                    sender_player_ref.set_role_state(self, RoleState::Consort(consort));
                }
            },
            ToServerPacket::SetForgerWill { role, will } => {
                if let RoleState::Forger(mut forger) = sender_player_ref.role_state(self).clone(){
                    forger.fake_role = role;
                    forger.fake_will = will;
                    sender_player_ref.set_role_state(self, RoleState::Forger(forger));
                }
            },
            ToServerPacket::SetAuditorChosenOutline { index } => {
                if !sender_player_ref.alive(self) {break 'packet_match;}

                if let RoleState::Auditor(mut auditor) = sender_player_ref.role_state(self).clone(){

                    if auditor.chosen_outline.is_some_and(|f|f == index) {
                        auditor.chosen_outline = None;
                    }

                    if  !self.roles_to_players.get(index as usize).is_none() && 
                        !auditor.previously_given_results.iter().any(|(i, _)| *i == index)
                    {
                        auditor.chosen_outline = Some(index);
                    }

                    sender_player_ref.set_role_state(self, RoleState::Auditor(auditor));
                }
            },
            ToServerPacket::VoteFastForwardPhase { fast_forward } => {
                sender_player_ref.set_fast_forward_vote(self, fast_forward);
            }
            _ => {
                log!(fatal "Game"; "Unimplemented ToServerPacket: {incoming_packet:?}");
                unreachable!();
            }
        }}
        
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_repeating_data(self)
        }

    }
}