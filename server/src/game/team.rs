use super::{player::PlayerReference};

#[derive(Debug, PartialEq, Eq)]
pub enum Team{
    Mafia, Coven, Vampire
}
#[derive(Debug, PartialEq, Eq)]
pub enum TeamState{
    Mafia,
    Coven{
        player_with_necronomicon: PlayerReference
    },
    Vampire{
        youngest_vampire: PlayerReference
    }
}
impl TeamState{
    pub fn team(&self)->Team{
        match self {
            TeamState::Mafia => Team::Mafia,
            TeamState::Coven { .. } => Team::Coven,
            TeamState::Vampire { .. } => Team::Vampire,
        }
    }
    pub fn same_team(a: TeamState, b: TeamState)->bool{
        a.team() == b.team()
    }
}
