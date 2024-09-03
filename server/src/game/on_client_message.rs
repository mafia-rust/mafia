use std::collections::HashMap;

use crate::{packet::ToServerPacket, strings::TidyableString, log};

use super::{
    chat::{ChatGroup, ChatMessageVariant, MessageSender},
    event::on_fast_forward::OnFastForward,
    phase::{PhaseState, PhaseType},
    player::{PlayerIndex, PlayerReference},
    role::{kira::{Kira, KiraGuess}, mayor::Mayor, puppeteer::PuppeteerAction, Role, RoleState}, role_list::{Faction, RoleSet}, 
    spectator::spectator_pointer::{SpectatorIndex, SpectatorPointer},
    Game
};




impl Game {
    pub fn on_spectator_message(&mut self, sender_index: SpectatorIndex, incoming_packet: ToServerPacket){
        let sender_pointer = SpectatorPointer::new(sender_index);

        #[allow(clippy::single_match)]
        match incoming_packet {
            ToServerPacket::VoteFastForwardPhase { fast_forward } => {
                if sender_pointer.host(self) && fast_forward && !self.phase_machine.time_remaining.is_zero(){
                    OnFastForward::invoke(self);
                }
            },
            _ => {
            }
        }
    }
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
                sender_player_ref.set_selection(self, target_ref_list.clone());
                
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

                if let RoleState::Mayor(Mayor{revealed: true}) = whisperee_ref.role_state(self) {
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::MayorCantWhisper);
                    break 'packet_match;
                }
                if let RoleState::Mayor(Mayor{revealed: true}) = sender_player_ref.role_state(self) {
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::MayorCantWhisper);
                    break 'packet_match;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::BroadcastWhisper { whisperer: sender_player_index, whisperee: whispered_to_player_index });
                let message = ChatMessageVariant::Whisper { 
                    from_player_index: sender_player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                sender_player_ref.add_private_chat_message(self, message.clone());

                for player in PlayerReference::all_players(self){
                    if 
                        player.role(self) == Role::Informant ||
                        whisperee_ref == player
                    {
                        player.add_private_chat_message(self, message.clone());
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
            },
            ToServerPacket::SetKiraGuess{guesses} => {
                if let RoleState::Kira(mut kira) = sender_player_ref.role_state(self).clone(){

                    let mut new_guesses: HashMap<PlayerReference, KiraGuess> = HashMap::new();

                    for (player_ref, guess) in guesses {
                        if Kira::allowed_to_guess(sender_player_ref, player_ref, self){
                            new_guesses.insert(player_ref, guess);
                        }
                    }

                    kira.guesses = new_guesses;
                    sender_player_ref.set_role_state(self, RoleState::Kira(kira));
                    Kira::set_guesses(sender_player_ref, self);
                }
            },
            ToServerPacket::SetWildcardRole { role } => {

                if !self.settings.enabled_roles.contains(&role) {
                    break 'packet_match;
                }
                
                match sender_player_ref.role_state(self).clone() {
                    RoleState::Wildcard(mut wild_card) => {
                        wild_card.role = role;
                        sender_player_ref.set_role_state(self, RoleState::Wildcard(wild_card));
                    }
                    RoleState::TrueWildcard(mut true_wildcard) => {
                        true_wildcard.role = role;
                        sender_player_ref.set_role_state(self, RoleState::TrueWildcard(true_wildcard));
                    }
                    RoleState::MafiaSupportWildcard(mut mafia_wild_card) => {
                        if RoleSet::MafiaSupport.get_roles().contains(&role) {
                            mafia_wild_card.role = role;
                        }
                        sender_player_ref.set_role_state(self, RoleState::MafiaSupportWildcard(mafia_wild_card));
                    }
                    RoleState::FiendsWildcard(mut fiends_wild_card) => {
                        if role.faction() == Faction::Fiends {
                            fiends_wild_card.role = role;
                        }
                        sender_player_ref.set_role_state(self, RoleState::FiendsWildcard(fiends_wild_card));
                    }
                    _ => {}
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
            ToServerPacket::SetForgerWill { role, will } => {
                if let RoleState::Forger(mut forger) = sender_player_ref.role_state(self).clone(){
                    forger.fake_role = role;
                    forger.fake_will = will;
                    sender_player_ref.set_role_state(self, RoleState::Forger(forger));
                }
                else if let RoleState::Counterfeiter(mut counterfeiter) = sender_player_ref.role_state(self).clone(){
                    counterfeiter.fake_role = role;
                    counterfeiter.fake_will = will;
                    sender_player_ref.set_role_state(self, RoleState::Counterfeiter(counterfeiter));
                }
            },
            ToServerPacket::SetCounterfeiterAction {action} => {
                if let RoleState::Counterfeiter(mut counterfeiter) = sender_player_ref.role_state(self).clone(){
                    counterfeiter.action = action;
                    sender_player_ref.set_role_state(self, RoleState::Counterfeiter(counterfeiter));
                }
            },
            ToServerPacket::SetAuditorChosenOutline { index } => {
                if !sender_player_ref.alive(self) {break 'packet_match;}

                if let RoleState::Auditor(mut auditor) = sender_player_ref.role_state(self).clone(){

                    if auditor.chosen_outline.is_some_and(|f|f == index) {
                        auditor.chosen_outline = None;
                    }

                    if  self.roles_to_players.get(index as usize).is_some() && 
                        !auditor.previously_given_results.iter().any(|(i, _)| *i == index)
                    {
                        auditor.chosen_outline = Some(index);
                    }

                    sender_player_ref.set_role_state(self, RoleState::Auditor(auditor));
                }
            },
            ToServerPacket::SetOjoAction { action } => {
                if let RoleState::Ojo(mut ojo) = sender_player_ref.role_state(self).clone(){
                    ojo.chosen_action = action.clone();
                    sender_player_ref.set_role_state(self, RoleState::Ojo(ojo));
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::OjoActionChosen { action });
                }
            },
            ToServerPacket::SetPuppeteerAction { action } => {
                if let RoleState::Puppeteer(mut pup) = sender_player_ref.role_state(self).clone(){
                    pup.action = action.clone();
                    if pup.marionettes_remaining == 0 {
                        pup.action = PuppeteerAction::Poison;
                    }
                    sender_player_ref.set_role_state(self, RoleState::Puppeteer(pup));
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::PuppeteerActionChosen { action });
                    
                    //Updates selection if it was invalid
                    sender_player_ref.set_selection(self, sender_player_ref.selection(self).clone());
                }
            },
            ToServerPacket::SetErosAction { action } => {
                if let RoleState::Eros(mut eros) = sender_player_ref.role_state(self).clone(){
                    eros.action = action.clone();
                    sender_player_ref.set_role_state(self, RoleState::Eros(eros));
                    sender_player_ref.add_private_chat_message(self, ChatMessageVariant::ErosActionChosen{ action });

                    //Updates selection if it was invalid
                    sender_player_ref.set_selection(self, sender_player_ref.selection(self).clone());
                }
            },
            ToServerPacket::VoteFastForwardPhase { fast_forward } => {
                sender_player_ref.set_fast_forward_vote(self, fast_forward);
            },
            ToServerPacket::ForfeitVote { forfeit } => {
                if 
                    self.current_phase().phase() == PhaseType::Discussion &&
                    sender_player_ref.alive(self)
                {
                    sender_player_ref.set_forfeit_vote(self, forfeit);
                }
            }
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

    }
}