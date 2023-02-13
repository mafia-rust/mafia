use tokio::sync::mpsc::UnboundedSender;

use crate::game::Game;
use crate::game::phase::{Phase, PhaseType};
use crate::game::role::{Role, RoleData};
use crate::game::phase_resetting::PhaseResetting;
use crate::network::packet::ToClientPacket;

pub type PlayerIndex = usize;

pub struct Player {
    name: String,
    index: PlayerIndex,
    role_data: RoleData,
    alive: bool,

    sender: UnboundedSender<ToClientPacket>,

    // Night phase variables
    alive_tonight:  PhaseResetting<bool>,
    died:           PhaseResetting<bool>,
    attacked:       PhaseResetting<bool>,
    roleblocked:    PhaseResetting<bool>,
    defense:        PhaseResetting<u8>,
    suspicious:     PhaseResetting<bool>,

    janitor_cleaned: PhaseResetting<bool>,
    forger: PhaseResetting<Option<(Role, String)>>, //this is new, maybe a bad idea? I dotn know, in old code this was ShownRole, ShownWill, ShownNote,
    disguised_as:   PhaseResetting<PlayerIndex>,

    targets: PhaseResetting<Vec<PlayerIndex>>,

    chosen_targets:   PhaseResetting<Vec<PlayerIndex>>, //Vec is not copy

    //Voting
    chosen_vote: PhaseResetting<PlayerIndex>,
    //judgement
    chosen_judgement: PhaseResetting<Verdict>  //need judgement enum
    // TODO
}

impl Player {
    pub fn new(index: PlayerIndex, name: String, sender: UnboundedSender<ToClientPacket>, role: Role) -> Self {
        Player {
            name,
            index,
            role_data: role.default_data(),
            alive: true,

            sender,

            alive_tonight:  PhaseResetting::new(true,  |p| p.alive, PhaseType::Night),
            died:           PhaseResetting::new(false, |_| false, PhaseType::Night),
            attacked:       PhaseResetting::new(false, |_| false, PhaseType::Night),
            roleblocked:    PhaseResetting::new(false, |_| false, PhaseType::Night),
            defense:        PhaseResetting::new(role.get_defense(), |p| p.get_role().get_defense(), PhaseType::Night),
            suspicious:     PhaseResetting::new(role.is_suspicious(), |p| p.get_role().is_suspicious(), PhaseType::Night),

            disguised_as:   PhaseResetting::new(index, |p| p.index, PhaseType::Night),
            janitor_cleaned:PhaseResetting::new(false, |_| false, PhaseType::Night),
            forger: todo!(),
            chosen_targets: PhaseResetting::new(vec![], |_| vec![], PhaseType::Night),
            targets: todo!(),

            chosen_vote: todo!(),
            chosen_judgement: todo!(),
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
}