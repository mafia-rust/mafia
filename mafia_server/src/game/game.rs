use std::collections::HashMap;
use super::grave::Grave;
use super::phase::Phase;
use super::player::Player;
use super::player::PlayerID;

pub struct Game {
    players: Vec<Player>,   // PlayerID is the index into this vec
    graves: Vec<Grave>,

    //RoleList
    //PhaseTimes
    //Investigator Results
    //other settings

    //these next 2 might want to be both combined into a single struct
    phase : Phase,
}