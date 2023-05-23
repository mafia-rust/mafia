

use crate::{game::{Game, available_buttons::AvailableButtons}, packet::ToClientPacket};

use super::{Player, PlayerReference};

impl PlayerReference{
    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        self.deref(game).sender.send(packet);
    }
    pub fn send_repeating_data(&self, game: &mut Game){
        self.send_chat_messages(game);
        self.send_available_buttons(game);
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
    fn requeue_chat_messages(&self, game: &mut Game){
        for msg in self.deref(game).chat_messages.clone().into_iter(){
            self.deref_mut(game).queued_chat_messages.push(msg);
        };
    }   

    fn send_available_buttons(&self, game: &mut Game){
        let new_buttons = AvailableButtons::from_player(game, *self);
        if new_buttons == self.deref(game).last_sent_buttons{
            return;
        }
        
        self.send_packet(game, ToClientPacket::YourButtons { buttons: new_buttons.clone() });
        self.deref_mut(game).last_sent_buttons = new_buttons
    }

}

