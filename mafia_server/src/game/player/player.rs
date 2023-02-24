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
    network::packet::ToClientPacket
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
    }
    fn send_chat_messages(&mut self){
        if self.queued_chat_messages.len() == 0{
            return;
        }
        
        
        //self.queued_chat_messages.pop()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}





