use std::collections::HashMap;

use crate::{packet::{ToServerPacket, ToClientPacket}, strings::TidyableString, log};

use super::{Game, player::{PlayerIndex, PlayerReference}, phase::{PhaseType, PhaseState}, chat::{ChatGroup, ChatMessage, MessageSender}, role::{Role, RoleState}};




impl Game {
    pub fn on_client_message(&mut self, sender_player_index: PlayerIndex, incoming_packet: ToServerPacket){

        let sender_player_ref = match PlayerReference::new(self, sender_player_index){
            Ok(sender_player_ref) => sender_player_ref,
            Err(_) => {
                log!(error "Game"; "Recieved message from invalid player index: {}", sender_player_index);
                return;
            }
        };

        'packet_match: {match incoming_packet {
            ToServerPacket::Vote { player_index: player_voted_index } => {
                let &PhaseState::Voting { trials_left } = self.current_phase() else {break 'packet_match};

                let player_voted_ref = match PlayerReference::index_option_to_ref(self, &player_voted_index){
                    Ok(player_voted_ref) => player_voted_ref,
                    Err(_) => break 'packet_match,
                };

                let vote_changed_succesfully = sender_player_ref.set_chosen_vote(self, player_voted_ref, true);

                if !vote_changed_succesfully {break 'packet_match;}

                //get all votes on people
                let mut living_players_count = 0;
                let mut voted_for_player: HashMap<PlayerReference, u8> = HashMap::new();


                for any_player_ref in PlayerReference::all_players(self){
                    if any_player_ref.alive(self){
                        living_players_count+=1;

                        if let Some(any_player_voted_ref) = any_player_ref.chosen_vote(self){

                            let mut voting_power = 1;
                            if let RoleState::Mayor(mayor) = any_player_ref.role_state(self).clone(){
                                if mayor.revealed {
                                    voting_power += 2;
                                }
                            }

                            if let Some(num_votes) = voted_for_player.get_mut(&any_player_voted_ref){
                                
                                *num_votes+=voting_power;
                            }else{
                                voted_for_player.insert(any_player_voted_ref, voting_power);
                            }
                        }
                    }
                }
                
                self.send_packet_to_all(
                    ToClientPacket::PlayerVotes { voted_for_player: 
                        PlayerReference::ref_map_to_index(voted_for_player.clone())
                    }
                );
                
                //if someone was voted
                let mut next_player_on_trial = None;
                for (player_with_votes_ref, num_votes) in voted_for_player.iter(){
                    if *num_votes > (living_players_count / 2){
                        next_player_on_trial = Some(*player_with_votes_ref);
                        break;
                    }
                }
                
                if let Some(player_on_trial) = next_player_on_trial {
                    self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_on_trial.index() } );
                    self.start_phase(PhaseState::Testimony { trials_left, player_on_trial });
                }
            },
            ToServerPacket::Judgement { verdict } => {
                if self.current_phase().phase() != PhaseType::Judgement {break 'packet_match;}
                
                sender_player_ref.set_verdict(self, verdict, true);
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
                
                //Send targeted message to chat groups
                sender_player_ref.get_current_send_chat_groups(self)
                    .into_iter().for_each(|chat_group| {
                        match sender_player_ref.role_state(self) {
                            // TODO: Role specific code here
                            RoleState::Jailor(_) => self.add_message_to_chat_group(
                                chat_group,
                                ChatMessage::JailorDecideExecute {
                                    targets: PlayerReference::ref_vec_to_index(&target_ref_list)
                                }
                            ),
                            _ => self.add_message_to_chat_group(
                                chat_group,
                                ChatMessage::Targeted { 
                                    targeter: sender_player_ref.index(), 
                                    targets: PlayerReference::ref_vec_to_index(&target_ref_list)
                                }
                            )
                        }
                    });
                //if no chat groups then send to self at least
                if sender_player_ref.get_current_send_chat_groups(self).is_empty(){
                    sender_player_ref.add_chat_message(self, ChatMessage::Targeted { 
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

                    //TODO possibly move message_sender
                    let message_sender = 
                    if sender_player_ref.role(self) == Role::Jailor && chat_group == ChatGroup::Jail {
                        MessageSender::Jailor
                    } else if 
                        sender_player_ref.role(self) == Role::Medium && 
                        (
                            (sender_player_ref.alive(self) && chat_group == ChatGroup::Dead) || 
                            (!sender_player_ref.alive(self) && chat_group==ChatGroup::Seance)
                        )
                    {
                        MessageSender::Medium
                    } else {
                        MessageSender::Player {player: sender_player_index}
                    };

                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        //TODO message sender, Jailor & medium

                        ChatMessage::Normal{
                            message_sender,
                            text: text.trim_newline().trim_whitespace().truncate(400).truncate_lines(20), 
                            chat_group
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

                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::BroadcastWhisper { whisperer: sender_player_index, whisperee: whispered_to_player_index });
                let message = ChatMessage::Whisper { 
                    from_player_index: sender_player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                whisperee_ref.add_chat_message(self, message.clone());
                sender_player_ref.add_chat_message(self, message.clone());

                for player in PlayerReference::all_players(self){
                    if player.role(self) == Role::Consigliere {
                        whisperee_ref.add_chat_message(self, message.clone());
                    }
                }
            },
            ToServerPacket::SaveWill { will } => {
                sender_player_ref.set_will(self, will);
            },
            ToServerPacket::SaveNotes { notes } => {
                sender_player_ref.set_notes(self, notes);
            },
            ToServerPacket::SaveDeathNote { death_note } => {
                sender_player_ref.set_death_note(self, death_note);
            },
            ToServerPacket::Leave => {
                sender_player_ref.leave(self);
            }
            ToServerPacket::SetDoomsayerGuess { guesses } => {
                if let RoleState::Doomsayer(mut doomsayer) = sender_player_ref.role_state(self).clone(){
                    doomsayer.guesses = guesses;
                    sender_player_ref.set_role_state(self, RoleState::Doomsayer(doomsayer));
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

    }
}