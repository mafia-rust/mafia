use crate::game::Game;
use crate::game::phase::{Phase, PhaseType};
use crate::game::role::{Role, RoleData};


use crate::game::phase_resetting::PhaseResetting;

pub type PlayerID = usize;

pub struct Player {
    name: String,
    id: PlayerID,
    role_data: RoleData,
    alive: bool,

    // Night phase variables
    alive_tonight:  PhaseResetting<bool>,
    died:           PhaseResetting<bool>,
    attacked:       PhaseResetting<bool>,
    roleblocked:    PhaseResetting<bool>,
    defense:        PhaseResetting<u8>,
    suspicious:     PhaseResetting<bool>,
    disguised_as:   PhaseResetting<PlayerID>,

    // Morning
    // TODO
    

}

impl Player {
    pub fn new(name: String, id: PlayerID, role: Role) -> Self {
        Player {
            name,
            id,
            role_data: role.default_data(),
            alive: true,

            alive_tonight:  PhaseResetting::new(true,  |p| p.alive, PhaseType::Night),
            died:           PhaseResetting::new(false, |_| false, PhaseType::Night),
            attacked:       PhaseResetting::new(false, |_| false, PhaseType::Night),
            roleblocked:    PhaseResetting::new(false, |_| false, PhaseType::Night),
            defense:        PhaseResetting::new(role.get_defense(), |p| p.get_role().get_defense(), PhaseType::Night),
            suspicious:     PhaseResetting::new(role.is_suspicious(), |p| p.get_role().is_suspicious(), PhaseType::Night),
            disguised_as:   PhaseResetting::new(id, |p| p.id, PhaseType::Night),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_role(&self) -> Role {
        self.role_data.role()
    }
}