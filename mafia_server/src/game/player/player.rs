use tokio::sync::mpsc::UnboundedSender;

use crate::game::Game;
use crate::game::chat::ChatMessage;
use crate::game::phase::{Phase, PhaseType};
use crate::game::role::{Role, RoleData};
use crate::game::phase_resetting::PhaseResetting;
use crate::game::visit::Visit;
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

    // Night phase variables
    pub alive_tonight:  PhaseResetting<bool>,
    pub died:           PhaseResetting<bool>,
    pub attacked:       PhaseResetting<bool>,
    pub roleblocked:    PhaseResetting<bool>,
    pub defense:        PhaseResetting<u8>,
    pub suspicious:     PhaseResetting<bool>,

    pub janitor_cleaned: PhaseResetting<bool>,
    //forger: PhaseResetting<Option<(Role, String)>>, //this is new, maybe a bad idea? I dotn know, in old code this was ShownRole, ShownWill, ShownNote,
    pub disguised_as:   PhaseResetting<PlayerIndex>,

    pub chosen_targets: PhaseResetting<Vec<PlayerIndex>>,//vec is not copy
    pub visits: PhaseResetting<Vec<Visit>>,

    //Voting
    pub chosen_vote: PhaseResetting<Option<PlayerIndex>>,
    //judgement
    pub chosen_judgement: PhaseResetting<i32>  //need judgement enum TODO verdict
    // TODO
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

            alive_tonight:  PhaseResetting::new(true,  |p| p.alive, PhaseType::Night),
            died:           PhaseResetting::new(false, |_| false, PhaseType::Night),
            attacked:       PhaseResetting::new(false, |_| false, PhaseType::Night),
            roleblocked:    PhaseResetting::new(false, |_| false, PhaseType::Night),
            defense:        PhaseResetting::new(role.get_defense(), |p| p.get_role().get_defense(), PhaseType::Night),
            suspicious:     PhaseResetting::new(role.is_suspicious(), |p| p.get_role().is_suspicious(), PhaseType::Night),

            disguised_as:   PhaseResetting::new(index, |p| p.index, PhaseType::Night),
            janitor_cleaned:PhaseResetting::new(false, |_| false, PhaseType::Night),
            //forger: todo!(),

            chosen_targets: PhaseResetting::new(vec![], |_| vec![], PhaseType::Night),
            visits:         PhaseResetting::new(vec![], |_| vec![], PhaseType::Night),  

            //Vote
            chosen_vote:    PhaseResetting::new(None, |_| None, PhaseType::Voting),
            //Judgement
            chosen_judgement: PhaseResetting::new(0, |_| 0, PhaseType::Judgement),//TODO enum not i32
            
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
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}





