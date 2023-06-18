use std::time::Duration;

use crate::{game::{Game, available_buttons::AvailableButtons}, packet::{ToClientPacket}, websocket_connections::connection::ClientSender};

use super::{PlayerReference, ClientConnection, DISCONNECT_TIMER_SECS};

impl PlayerReference{
    pub fn reconnect(&self, game: &mut Game, sender: ClientSender){
        self.deref_mut(game).connection = ClientConnection::Connected(sender);
        self.requeue_chat_messages(game);
    }
    pub fn lose_connection(&self, game: &mut Game){
        self.deref_mut(game).connection = ClientConnection::LostConnection { disconnect_timer: Duration::from_secs(DISCONNECT_TIMER_SECS) };
    }
    pub fn leave(&self, game: &mut Game) {
        self.deref_mut(game).connection = ClientConnection::Disconnected;
    }
    pub fn is_connected(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::Connected(_))
    }
    pub fn has_lost_connection(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::LostConnection {..})
    }
    pub fn has_left(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::Disconnected)
    }

    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        if let ClientConnection::Connected(sender) = &self.deref(game).connection{
            sender.send(packet);
        }
    }
    pub fn send_packets(&self, game: &Game, packets: Vec<ToClientPacket>){
        for packet in packets{
            self.send_packet(game, packet);
        }
    }
    pub fn send_repeating_data(&self, game: &mut Game){
        self.send_chat_messages(game);
        self.send_available_buttons(game);
    }

    pub fn send_chat_messages(&self, game: &mut Game){
        
        if self.deref(game).queued_chat_messages.is_empty() {
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
    #[allow(unused)]
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

