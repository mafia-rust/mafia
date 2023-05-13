use std::collections::HashMap;

use crate::network::packet::{ToServerPacket, ToClientPacket, YourButtons};

use super::{Game, player::{PlayerIndex, Player, PlayerReference, self}, phase::PhaseType, chat::{ChatGroup, ChatMessage, MessageSender}};




impl Game {
    pub fn on_client_message(&mut self, sender_player_index: PlayerIndex, incoming_packet: ToServerPacket){

        let sender_player_ref = match PlayerReference::new(self, sender_player_index){
            Ok(sender_player_ref) => sender_player_ref,
            Err(_) => {
                println!("Bad player index sent message");
                return;
            }
        };

        'packet_match: {match incoming_packet {
            ToServerPacket::Vote { player_index: mut player_voted_index } => {

                let player_voted_ref = match PlayerReference::index_option_to_ref(self, &player_voted_index){
                    Ok(player_voted_ref) => player_voted_ref,
                    Err(_) => break 'packet_match,
                };

                let vote_changed_succesfully = sender_player_ref.set_chosen_vote(self, player_voted_ref);

                if !vote_changed_succesfully {break 'packet_match;}

                //get all votes on people
                let mut living_players_count = 0;
                let mut voted_for_player: HashMap<PlayerReference, u8> = HashMap::new();


                for any_player_ref in PlayerReference::all_players(self){
                    if *any_player_ref.alive(self){
                        living_players_count+=1;

                        if let Some(any_player_voted_ref) = any_player_ref.chosen_vote(self){

                            if let Some(num_votes) = voted_for_player.get_mut(any_player_voted_ref){
                                *num_votes+=1;
                            }else{
                                voted_for_player.insert(*any_player_voted_ref, 1);
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
                    if *num_votes >= 1+(living_players_count / 2){
                        next_player_on_trial = Some(*player_with_votes_ref);
                        break;
                    }
                }
                
                if let Some(next_player_on_trial_unwrap) = next_player_on_trial{
                    self.player_on_trial = next_player_on_trial;

                    self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: *next_player_on_trial_unwrap.index() } );
                    self.jump_to_start_phase(PhaseType::Testimony);
                }
            },
            ToServerPacket::Judgement { verdict } => {
                sender_player_ref.set_verdict(self, verdict);
            },
            ToServerPacket::Target { player_index_list } => {
                //TODO Send you targeted someone message in correct chat.
                let target_ref_list = match PlayerReference::index_vec_to_ref(self, &player_index_list){
                    Ok(target_ref_list) => target_ref_list,
                    Err(_) => {
                        break 'packet_match;
                    },
                };
                sender_player_ref.set_chosen_targets(self, target_ref_list);
            },
            ToServerPacket::DayTarget { player_index } => {               
                let target_ref = match PlayerReference::new(self, player_index){
                    Ok(target_ref) => target_ref,
                    Err(_) => break 'packet_match,
                };
                if sender_player_ref.role(self).can_day_target(self, sender_player_ref, target_ref){
                    sender_player_ref.role(self).do_day_action(self, sender_player_ref, target_ref);
                }
            },
            ToServerPacket::SendMessage { text } => {

                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    break 'packet_match;
                }
                
                for chat_group in sender_player_ref.role(self).get_current_send_chat_groups(self, sender_player_ref){
                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        //TODO message sender, Jailor & medium
                        ChatMessage::Normal { message_sender: MessageSender::Player {player: sender_player_index} , text: text.clone(), chat_group }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {

                let whisperee_ref = match PlayerReference::new(self, whispered_to_player_index){
                    Ok(whisperee_ref) => whisperee_ref,
                    Err(_) => break 'packet_match,
                };

                //ensure its day and your not whispering yourself and the other player exists
                if !self.current_phase().is_day() || whisperee_ref == sender_player_ref{
                    break 'packet_match;
                }
                
                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    break 'packet_match;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::BroadcastWhisper { whisperer: sender_player_index, whisperee: whispered_to_player_index });
                let message = ChatMessage::Whisper { 
                    from_player_index: sender_player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                whisperee_ref.add_chat_message(self, message.clone());
                sender_player_ref.add_chat_message(self, message);

                //TODO, send to blackmailer
            },
            ToServerPacket::SaveWill { will } => {
                sender_player_ref.set_will(self, will);
            },
            ToServerPacket::SaveNotes { notes } => {
                sender_player_ref.set_notes(self, notes);
            },
            _ => unreachable!()
        }}
        
        let packet = ToClientPacket::YourButtons { buttons: YourButtons::from(self, sender_player_ref)};
        sender_player_ref.send_packet(self, packet);

        for player_ref in PlayerReference::all_players(self){
            player_ref.send_chat_messages(self);
        }

    }
}