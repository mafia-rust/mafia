use std::collections::HashSet;

use crate::game::player::PlayerReference;



pub struct VoteToEndPhase{
    players_voted: HashSet<PlayerReference>,
}

impl VoteToEndPhase{
    fn on_player_voted(&mut self, player: PlayerReference){
        if self.players_voted.contains(&player){
            self.players_voted.remove(&player);
        }else{
            self.players_voted.insert(player);
        }
    }
    fn is_vote_skip(&self, players: &HashSet<PlayerReference>)->bool{
        self.players_voted.len() == players.len()
    }
}