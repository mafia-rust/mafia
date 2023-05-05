

use crate::{network::packet::{YourButtons, ToClientPacket}, game::Game};

use super::{Player, PlayerReference};

impl Player{
    pub fn send_packet(&self, packet: ToClientPacket){
        self.sender.send(packet);
    }
    fn requeue_chat_messages(&mut self){
        for msg in self.chat_messages.iter(){
            self.queued_chat_messages.push(msg.clone());
        }
    }
    pub fn send_chat_messages(&mut self){
        
        if self.queued_chat_messages.len() == 0 {
            return;
        }
        
        let mut chat_messages_out = vec![];

        //get the first 5
        for _ in 0..5{
            let msg_option = self.queued_chat_messages.get(0);
            if let Some(msg) = msg_option{
                chat_messages_out.push(msg.clone());
                self.queued_chat_messages.remove(0);
            }else{ break; }
        }
        
        self.send_packet(ToClientPacket::AddChatMessages { chat_messages: chat_messages_out });
        

        self.send_chat_messages();
    }
    fn send_available_buttons(game: &mut Game, player_ref: PlayerReference){


        //TODO maybe find a way to check to see if we should send this like i do in chat messages
        player_ref.deref(game).send_packet(ToClientPacket::YourButtons { buttons: PlayerReference::all_players(game).iter().map(|other_player_ref|{
            YourButtons{
                vote: false,
                target: player_ref.deref(game).role().can_night_target(&game, player_ref, *other_player_ref),
                day_target: player_ref.deref(game).role().can_day_target(&game, player_ref, *other_player_ref),
            }
        }).collect()});
    }
}

