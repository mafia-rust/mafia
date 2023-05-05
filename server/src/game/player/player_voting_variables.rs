use crate::{game::{verdict::Verdict, Game}, prelude::MafiaError, network::packet::ToClientPacket};

use super::{Player, PlayerIndex};

pub struct PlayerVotingVariables{
    //Voting
    pub chosen_vote:    Option<PlayerIndex>,
    pub verdict:        Verdict,
}
impl PlayerVotingVariables{
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