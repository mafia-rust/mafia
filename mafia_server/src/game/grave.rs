use super::player::PlayerIndex;
use super::phase::Phase;
use super::role::Role;

#[derive(Clone)]
pub struct Grave {
    player: PlayerIndex,

    role: GraveRole,
    killer: GraveKiller,
    will: String,

    died_phase: GravePhase,
    day_number: u8,
}

#[derive(Clone)]
pub enum GraveRole {
    Cleaned,
    Stoned,
    Role(Role),
}

#[derive(Clone)]
pub enum GraveKiller {
    Lynching, 
    Mafia,
    Role(Role)
}

#[derive(Clone)]
pub enum GravePhase {
    Day, Night
}