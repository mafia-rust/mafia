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

    // Night phase variables
    pub alive_tonight:  PhaseResetting<bool>,
    pub died:           PhaseResetting<bool>,
    pub attacked:       PhaseResetting<bool>,
    pub roleblocked:    PhaseResetting<bool>,
    pub defense:        PhaseResetting<u8>,
    pub suspicious:     PhaseResetting<bool>,

    pub janitor_cleaned:PhaseResetting<bool>,
    //forger: PhaseResetting<Option<(Role, String)>>, //this is new, maybe a bad idea? I dotn know, in old code this was ShownRole, ShownWill, ShownNote,
    pub disguised_as:   PhaseResetting<PlayerIndex>,

    pub chosen_targets: PhaseResetting<Vec<PlayerIndex>>,
    pub visits:         PhaseResetting<Vec<Visit>>,

    //Voting
    pub chosen_vote:    PhaseResetting<Option<PlayerIndex>>,
    pub verdict:        PhaseResetting<Verdict>
}

impl Player {
    pub fn new(index: PlayerIndex, name: String, sender: UnboundedSender<ToClientPacket>, role: Role) -> Self {
        // Cry? Maybe? Want to cry about this unwrap? That's unfortunate. I'm sorry.
        macro_rules! this {($g:ident) => {$g.get_player(index).unwrap()}};
        Player {
            name,
            index,
            role_data: role.default_data(),
            alive: true,

            chat_messages: Vec::new(),
            queued_chat_messages: Vec::new(),

            sender,

            alive_tonight:  PhaseResetting::new(PhaseType::Night, &move |g| this!(g).alive),
            died:           PhaseResetting::new(PhaseType::Night, &move |_| false),
            attacked:       PhaseResetting::new(PhaseType::Night, &move |_| false),
            roleblocked:    PhaseResetting::new(PhaseType::Night, &move |_| false),
            defense:        PhaseResetting::new(PhaseType::Night, &move |g| this!(g).get_role().get_defense()),
            suspicious:     PhaseResetting::new(PhaseType::Night, &move |g| this!(g).get_role().is_suspicious()),

            disguised_as:   PhaseResetting::new(PhaseType::Night, &move |_| index),
            janitor_cleaned:PhaseResetting::new(PhaseType::Night, &move |_| false),
            //forger: todo!(),

            chosen_targets: PhaseResetting::new(PhaseType::Night, &move |_| vec![]),
            visits:         PhaseResetting::new(PhaseType::Night, &move |_| vec![]),  

            //Voting
            chosen_vote:    PhaseResetting::new(PhaseType::Voting, &move |_| None),
            verdict:        PhaseResetting::new(PhaseType::Judgement, &move |_| Verdict::Abstain),
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





