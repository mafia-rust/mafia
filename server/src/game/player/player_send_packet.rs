

use crate::{network::packet::{YourButtons, ToClientPacket}, game::Game};

use super::{Player, PlayerReference};

impl PlayerReference{
    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        self.deref(game).sender.send(packet);
    }
    fn requeue_chat_messages(&self, game: &mut Game){
        for msg in self.deref(game).chat_messages.clone().into_iter(){
            self.deref_mut(game).queued_chat_messages.push(msg);
        };
    }
    pub fn send_chat_messages(&self, game: &mut Game){
        
        if self.deref(game).queued_chat_messages.len() == 0 {
            return;
        }
        
        let mut chat_messages_out = vec![];

        //get the first 5
        for _ in 0..5{
            let msg_option = self.deref(game).queued_chat_messages.get(0);
            if let Some(msg) = msg_option{
                chat_messages_out.push(msg.clone());
                self.deref_mut(game).queued_chat_messages.remove(0);
            }else{ break; }
        }
        
        self.send_packet(game, ToClientPacket::AddChatMessages { chat_messages: chat_messages_out });
        

        self.send_chat_messages(game);
    }
    fn send_available_buttons(&self, game: &mut Game){


        //TODO maybe find a way to check to see if we should send this like i do in chat messages
        self.send_packet(game, ToClientPacket::YourButtons { buttons: PlayerReference::all_players(game).iter().map(|other_player_ref|{
            YourButtons{
                vote: false,
                target: self.role(game).can_night_target(&game, *self, *other_player_ref),
                day_target: self.role(game).can_day_target(&game, *self, *other_player_ref),
            }
        }).collect()});
    }
}

