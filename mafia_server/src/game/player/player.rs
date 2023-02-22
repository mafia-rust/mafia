use tokio::sync::mpsc::UnboundedSender;

use crate::game::Game;
use crate::game::chat::ChatMessage;
use crate::game::phase::{Phase, PhaseType};
use crate::game::role::{Role, RoleData};
use crate::game::phase_resetting::PhaseResetting;
use crate::game::visit::Visit;
use crate::game::vote::Verdict;
use crate::network::packet::ToClientPacket;

pub type PlayerIndex = usize;

pub struct Player {
    pub name: String,
    pub index: PlayerIndex,
    pub role_data: RoleData,
    pub alive: bool,

    sender: UnboundedSender<ToClientPacket>,

    chat_messages: Vec<ChatMessage>,
    queued_chat_messages: Vec<ChatMessage>,

    //PHASE RESETTING VARIABLES

    // Night phase variables TODO possibly rename these variables, maybe not?
    pub alive_tonight:  bool,
    pub died:           bool,
    pub attacked:       bool,
    pub roleblocked:    bool,
    pub defense:        u8,    
    pub suspicious:     bool,

    pub janitor_cleaned:bool,
    //forger: Option<(Role, String)>, //this is new, maybe a bad idea? I dotn know, in old code this was ShownRole, ShownWill, ShownNote,
    pub disguised_as:   PlayerIndex,

    pub chosen_targets: Vec<PlayerIndex>,
    pub visits:         Vec<Visit>,

    //Voting
    pub chosen_vote:    Option<PlayerIndex>,
    pub verdict:        Verdict
}

impl Player {
    pub fn new(index: PlayerIndex, name: String, sender: UnboundedSender<ToClientPacket>, role: Role) -> Self {
        Player {
            name,
            index,
            role_data: role.default_data(),
            alive: true,

            chat_messages: Vec::new(),
            queued_chat_messages: Vec::new(),

            sender,

            alive_tonight:  true,
            died:           false,
            attacked:       false,
            roleblocked:    false,
            defense:        role.get_defense(),
            suspicious:     role.is_suspicious(),

            disguised_as:   index,
            janitor_cleaned:false,
            //forger: todo!(),

            chosen_targets: vec![],
            visits:         vec![],  

            //Voting
            chosen_vote:    None,
            verdict:        Verdict::Abstain,
        }
    }

    pub fn send(&self, packet: ToClientPacket){
        self.sender.send(packet);
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
    pub fn send_chat_messages(&mut self){
        //recursive
        
        //self.queued_chat_messages.pop()
    }

    pub fn set_night(&mut self){
        self.alive_tonight = self.alive;
        self.died =          false;
        self.attacked =      false;
        self.roleblocked =   false;
        self.defense =       self.get_role().get_defense();
        self.suspicious =    self.get_role().is_suspicious();

        self.disguised_as =  self.index;
        self.janitor_cleaned=false;
        //forger: todo!(),

        self.chosen_targets= vec![];
        self.visits=         vec![];  

    }
    pub fn set_voting(&mut self){
        self.chosen_vote = None;
        self.verdict = Verdict::Abstain;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}





