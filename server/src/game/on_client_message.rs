use std::collections::HashMap;

use crate::network::packet::{ToServerPacket, ToClientPacket, YourButtons};

use super::{Game, player::{PlayerIndex, Player, PlayerReference, self}, phase::PhaseType, chat::{ChatGroup, ChatMessage, MessageSender}};




impl Game {
    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet: ToServerPacket){

        let player_ref = match PlayerReference::new(self, player_index){
            Ok(player_ref) => player_ref,
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

                let vote_changed_succesfully = player_ref.set_chosen_vote(self, player_voted_ref);

                if !vote_changed_succesfully {break 'packet_match;}

                //get all votes on people
                let mut living_players_count = 0;
                let mut voted_for_player: HashMap<PlayerIndex, u8> = HashMap::new();


                for player_ref in PlayerReference::all_players(self){
                    if *player_ref.alive(self){
                        living_players_count+=1;

                        if let Some(player_voted) = player_ref.chosen_vote(self){
                            if let Some(num_votes) = voted_for_player.get_mut(player_voted.index()){
                                *num_votes+=1;
                            }else{
                                voted_for_player.insert(*player_voted.index(), 1);
                            }
                        }
                    }
                }
                
                self.send_packet_to_all(ToClientPacket::PlayerVotes { voted_for_player: voted_for_player.clone() });
                

                //if someone was voted
                let mut player_voted = None;
                for player_index in 0..(voted_for_player.len() as PlayerIndex){
                    if let Some(num_votes) = voted_for_player.get(&player_index){
                        if *num_votes >= 1+(living_players_count / 2){
                            player_voted = Some(player_index as u8);
                            break;
                        }
                    }
                }
                
                if let Some(player_voted_ref_unwrap) = player_voted_ref{
                    self.player_on_trial = player_voted_ref;

                    self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: *player_voted_ref_unwrap.index() } );
                    self.jump_to_start_phase(PhaseType::Testimony);
                }
            },
            ToServerPacket::Judgement { verdict } => {
                player_ref.set_verdict(self, verdict);
            },
            ToServerPacket::Target { player_index_list } => {
                //TODO Send you targeted someone message in correct chat.
                let target_ref_list = match PlayerReference::index_vec_to_ref(self, &player_index_list){
                    Ok(target_ref_list) => target_ref_list,
                    Err(_) => {
                        break 'packet_match;
                    },
                };
                player_ref.set_chosen_targets(self, target_ref_list);
            },
            ToServerPacket::DayTarget { player_index } => {               
                let target_ref = match PlayerReference::new(self, player_index){
                    Ok(target_ref) => target_ref,
                    Err(_) => break 'packet_match,
                };
                if player_ref.role(self).can_day_target(self, player_ref, target_ref){
                    player_ref.role(self).do_day_action(self, player_ref, target_ref);
                }
            },
            ToServerPacket::SendMessage { text } => {

                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    break 'packet_match;
                }
                
                for chat_group in player_ref.role(self).get_current_send_chat_groups(self, player_ref){
                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        //TODO message sender, Jailor & medium
                        ChatMessage::Normal { message_sender: MessageSender::Player {player: player_index} , text: text.clone(), chat_group }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {

                let whisperee_ref = match PlayerReference::new(self, whispered_to_player_index){
                    Ok(whisperee_ref) => whisperee_ref,
                    Err(_) => break 'packet_match,
                };

                //ensure its day and your not whispering yourself and the other player exists
                if !self.current_phase().is_day() || whisperee_ref == player_ref{
                    break 'packet_match;
                }
                
                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    break 'packet_match;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::BroadcastWhisper { whisperer: player_index, whisperee: whispered_to_player_index });
                let message = ChatMessage::Whisper { 
                    from_player_index: player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                whisperee_ref.add_chat_message(self, message.clone());
                player_ref.add_chat_message(self, message);

                //TODO, send to blackmailer
            },
            ToServerPacket::SaveWill { will } => {
                player_ref.set_will(self, will);
            },
            ToServerPacket::SaveNotes { notes } => {
                player_ref.set_notes(self, notes);
            },
            _ => unreachable!()
        }}
        
        let packet = ToClientPacket::YourButtons { buttons: YourButtons::from(self, player_ref)};
        player_ref.send_packet(self, packet);

        for player_ref in PlayerReference::all_players(self){
            player_ref.send_chat_messages(self);
        }

    }
}