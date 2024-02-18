
pub mod vote_skip_phase;
pub mod dead_can_vote;

use serde::{Deserialize, Serialize};

use self::vote_skip_phase::VoteToEndPhase;
use self::dead_can_vote::DeadCanVote;


#[derive(Default)]
pub struct Modifiers{
    modifiers: Vec<Modifier>
}
impl Modifiers{

}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Modifier{
    VoteToEndPhase,
    DeadCanVote
}

pub enum ModifierState{
    VoteToEndPhase(VoteToEndPhase),
    DeadCanVote(DeadCanVote)
}