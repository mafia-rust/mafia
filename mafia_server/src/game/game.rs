use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use crate::prelude::*;
use super::grave::Grave;
use super::phase::{Phase, PhaseStateMachine};
use super::player::{Player, PlayerIndex};

lazy_static!(
    pub static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
        players: Vec::new(),    //keep as vec because each player has a number like in real game
        graves: Vec::new(),     //order of graves
        phase_machine: PhaseStateMachine::new(),
    }));
);

pub struct Game {
    pub players: Vec<Player>,   // PlayerID is the index into this vec
    pub graves: Vec<Grave>,

    // pub role_list: Vec<Role>,
    // pub invesigator_results: TODO

    pub phase_machine : PhaseStateMachine,
}

impl Game {
    pub fn get_player(&self, id: PlayerIndex) -> Result<&Player> {
        self.players.get(id).ok_or_else(|| err!(generic, "Failed to get player {}", id))
    }

    pub fn get_player_mut(&mut self, id: PlayerIndex) -> Result<&mut Player> {
        self.players.get_mut(id).ok_or_else(|| err!(generic, "Failed to get player {}", id))
    }

    pub fn get_current_phase(&self) -> Phase {
        self.phase_machine.current_state
    }
}