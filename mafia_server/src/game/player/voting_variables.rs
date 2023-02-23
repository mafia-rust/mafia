use crate::game::vote::Verdict;

use super::{Player, PlayerIndex};

pub struct VotingVariables{
    //Voting
    pub chosen_vote:    Option<PlayerIndex>,
    pub verdict:        Verdict,
}
impl VotingVariables{
    pub fn new()->Self{
        Self{
            chosen_vote : None,
            verdict : Verdict::Abstain,
        }
    }
    pub fn reset(&mut self){
        self.chosen_vote = None;
        self.verdict = Verdict::Abstain;
    }
}