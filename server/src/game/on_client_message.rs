use crate::network::packet::{ToServerPacket, ToClientPacket, YourButtons};

use super::{Game, player::{PlayerIndex, Player}, phase::PhaseType, chat::{ChatGroup, ChatMessage, MessageSender}};




impl Game {
    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::Vote { player_index: mut player_voted_index } => {


                let vote_changed_succesfully = Player::set_chosen_vote(self, player_index, player_voted_index);

                if !vote_changed_succesfully {return;}

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
                            if let Some(num_votes) = voted_for_player.get_mut(*player_voted as usize){
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
                Player::set_verdict(self, player_index, verdict);
            },
            ToServerPacket::Target { player_index_list } => {
                //TODO Send you targeted someone message in correct chat.
                Player::set_chosen_targets(self, player_index, player_index_list);
            },
            ToServerPacket::DayTarget { player_index } => {
                //TODO can daytarget???
                //TODO
            },
            ToServerPacket::SendMessage { text } => {
                let player = self.get_unchecked_mut_player(player_index);

                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    return;
                }
                
                for chat_group in player.role().get_current_send_chat_groups(player_index, self){
                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        //TODO message sender, Jailor & medium
                        ChatMessage::Normal { message_sender: MessageSender::Player {player: player_index} , text: text.clone(), chat_group }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {

                //ensure its day and your not whispering yourself and the other player exists
                if !self.current_phase().is_day() || whispered_to_player_index == player_index || self.players.len() <= whispered_to_player_index as usize{
                    return;
                }
                
                if text.replace("\n", "").replace("\r", "").trim().is_empty() {
                    return;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::BroadcastWhisper { whisperer: player_index, whisperee: whispered_to_player_index });
                let message = ChatMessage::Whisper { 
                    from_player_index: player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                let to_player = self.get_unchecked_mut_player(whispered_to_player_index);
                to_player.add_chat_message(message.clone());

                let from_player = self.get_unchecked_mut_player(player_index);
                from_player.add_chat_message(message);
                

                //TODO, send to blackmailer
            },
            ToServerPacket::SaveWill { will } => {
                let player = self.get_unchecked_mut_player(player_index);
                player.set_will(will);
            },
            ToServerPacket::SaveNotes { notes } => {
                let player = self.get_unchecked_mut_player(player_index);
                player.set_notes(notes);
            },
            _ => unreachable!()
        }
        
        let packet = ToClientPacket::YourButtons { buttons: YourButtons::from(self, player_index)};
        self.get_unchecked_mut_player(player_index).send_packet(packet);

        for player in self.players.iter_mut(){
            player.send_chat_messages();
        }

    }
}