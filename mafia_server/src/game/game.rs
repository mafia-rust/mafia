use std::collections::HashMap;
use super::grave::Grave;
use super::phase::PhaseStateMachine;
use super::player::Player;
use super::player::PlayerID;
use super::roles::Role;

pub struct Game {
    players: Vec<Player>,   // PlayerID is the index into this vec
    graves: Vec<Grave>,

    //RoleList
    //PhaseTimes
    //Investigator Results
    //other settings

    phase_machine : PhaseStateMachine,
}