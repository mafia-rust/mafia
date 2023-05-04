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

                let vote_changed_succesfully = Player::set_chosen_vote(self, player_ref, player_voted_ref);

                if !vote_changed_succesfully {break 'packet_match;}

                //get all votes on people
                let mut living_players_count = 0;
                let mut voted_for_player: Vec<u8> = Vec::new();

                for _ in self.players.iter(){
                    voted_for_player.push(0);
                }

                for player in self.players.iter(){
                    if *player.alive(){
                        living_players_count+=1;

                        if let Some(player_voted) = player.chosen_vote(){
                            if let Some(num_votes) = voted_for_player.get_mut(*player_voted.index() as usize){
                                *num_votes+=1;
                            }
                        }
                    }
                }
                
                self.send_packet_to_all(ToClientPacket::PlayerVotes { voted_for_player: voted_for_player.clone() });

                //if someone was voted
                let mut player_voted = None;
                for player_index in 0..voted_for_player.len(){
                    let num_votes = voted_for_player[player_index];
                    if num_votes > (living_players_count / 2){
                        player_voted = Some(player_index as u8);
                        break;
                    }
                }
                

                if let Some(player_voted_index) = player_voted{
                    self.player_on_trial = player_voted;

                    self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_voted_index } );
                    self.jump_to_start_phase(PhaseType::Testimony);
                }
            },
            ToServerPacket::Judgement { verdict } => {
                Player::set_verdict(self, player_ref, verdict);
            },
            ToServerPacket::Target { player_index_list } => {
                //TODO Send you targeted someone message in correct chat.
                let target_ref_list = match PlayerReference::index_vec_to_ref(self, &player_index_list){
                    Ok(target_ref_list) => target_ref_list,
                    Err(_) => {
                        break 'packet_match;
                    },
                };
                Player::set_chosen_targets(self, player_ref, target_ref_list);
            },
            ToServerPacket::DayTarget { player_index } => {
                //TODO can daytarget???
                //TODO
            },
            ToServerPacket::SendMessage { text } => {
                let player = player_ref.deref(self);

                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    break 'packet_match;
                }
                
                for chat_group in player.role().get_current_send_chat_groups(self, player_ref){
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
        
                whisperee_ref.deref(self).add_chat_message(message.clone());
                player_ref.deref(self).add_chat_message(message);

                //TODO, send to blackmailer
            },
            ToServerPacket::SaveWill { will } => {
                player_ref.deref_mut(self).set_will(will);
            },
            ToServerPacket::SaveNotes { notes } => {
                player_ref.deref_mut(self).set_notes(notes);
            },
            _ => unreachable!()
        }}
        
        let packet = ToClientPacket::YourButtons { buttons: YourButtons::from(self, player_ref)};
        player_ref.deref(self).send_packet(packet);

        for player in self.players.iter_mut(){
            player.send_chat_messages();
        }

    }
}