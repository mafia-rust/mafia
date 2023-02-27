use serde::{Serialize, Deserialize};

use super::player::PlayerIndex;
use super::role::Role;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grave {
    player: PlayerIndex,

    role: GraveRole,
    killer: GraveKiller,
    will: String,

    died_phase: GravePhase,
    day_number: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GraveRole {
    Cleaned,
    Stoned,
    Role(Role),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GraveKiller {
    Lynching, 
    Mafia,
    Role(Role)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GravePhase {
    Day, Night
}