use tokio::sync::mpsc::UnboundedSender;

use crate::{
    game::{
        Game, chat::ChatMessage, 
        phase_resetting::PhaseResetting, visit::Visit, vote::Verdict,
        phase::{
            PhaseType
        }, 
        role::{
            Role, RoleData
        }, 
    }, 
    network::packet::{ToClientPacket, PlayerButtons}
};

use super::{player_voting_variables::PlayerVotingVariables, player_night_variables::PlayerNightVariables};

pub type PlayerIndex = usize;

pub struct Player {
    pub name: String,
    pub index: PlayerIndex,
    pub role_data: RoleData,
    pub alive: bool,

    sender: UnboundedSender<ToClientPacket>,

    chat_messages: Vec<ChatMessage>,
    queued_chat_messages: Vec<ChatMessage>, 

    night_variables: PlayerNightVariables,
    voting_variables: PlayerVotingVariables,
}

impl Player {
    pub fn new(index: PlayerIndex, name: String, sender: UnboundedSender<ToClientPacket>, role: Role) -> Self {
        Self {
            name,
            index,
            role_data: role.default_data(),
            alive: true,

            sender,
            chat_messages: Vec::new(),

            queued_chat_messages: Vec::new(),

            night_variables: PlayerNightVariables::new(),
            voting_variables: PlayerVotingVariables::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_role(&self) -> Role {
        self.role_data.role()
    }

    pub fn add_chat_message(&mut self, message: ChatMessage) {
        self.chat_messages.push(message.clone());
        self.queued_chat_messages.push(message);
    }
    
    pub fn reset_phase_variables(&mut self, phase: PhaseType){
        match phase {
            PhaseType::Morning => {},
            PhaseType::Discussion => {},
            PhaseType::Voting => self.voting_variables.reset(),
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => self.voting_variables.reset(),
        }
    }


    //sync server client
    pub fn send(&self, packet: ToClientPacket){
        self.sender.send(packet);
    }
    ///call this repeadedly
    pub fn syncromize_server_to_client(&mut self){
        self.send_chat_messages();
        // self.send_available_buttons();
    }
    
    fn send_chat_messages(&mut self){
        if self.queued_chat_messages.len() == 0{    //redundant with the next if in the send. possibly delete this
            return;
        }
        
        let mut chat_messages_out = vec![];
        //get the first 5 messages and send them
        for i in 0..5{
            let msg_option = self.queued_chat_messages.get(i);
            if let Some(msg) = msg_option{
                chat_messages_out.push(msg.clone());
            }else{ break; }
        }
        
        if chat_messages_out.len() > 0{
            self.send(ToClientPacket::AddChatMessages { chat_messages: chat_messages_out });
        }

        self.send_chat_messages();
    }
    
    fn send_available_buttons(&mut self, game: &Game){

        //TODO maybe find a way to check to see if we should send this like i do in chat messages
        
        self.send(ToClientPacket::PlayerButtons { buttons: game.players.iter().map(|player|{
            PlayerButtons{
                vote: false,
                target: self.get_role().can_night_target(self.index, player.index, game),
                day_target: self.get_role().can_day_target(self.index, player.index, game),
            }
        }).collect() });
    }

}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}





