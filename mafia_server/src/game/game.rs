use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use crate::prelude::*;
use super::grave::Grave;
use super::phase::{Phase, PhaseStateMachine};
use super::player::{Player, PlayerID};
use super::role::Role;

lazy_static!(
    pub static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
        players: Vec::new(),
        graves: Vec::new(),
        phase_machine: PhaseStateMachine::new(),
    }));
);

pub struct Game {
    pub players: Vec<Player>,   // PlayerID is the index into this vec
    pub graves: Vec<Grave>,

    //RoleList
    //PhaseTimes
    //Investigator Results
    //other settings

    pub phase_machine : PhaseStateMachine,
}

impl Game {
    pub fn get_player(&self, id: PlayerID) -> Result<&Player> {
        self.players.get(id).ok_or(err!(generic, "Failed to get player {}", id))
    }

    pub fn get_player_mut(&mut self, id: PlayerID) -> Result<&mut Player> {
        self.players.get_mut(id).ok_or(err!(generic, "Failed to get player {}", id))
    }

    pub fn get_current_phase(&self) -> Phase {
        self.phase_machine.current_state
    }
}