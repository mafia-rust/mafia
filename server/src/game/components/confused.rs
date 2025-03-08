use std::collections::HashSet;

use crate::game::{player::PlayerReference, Game};


#[derive(Default)]
pub struct Confused{
    players: HashSet<PlayerReference>
}

impl Game {
    fn confused(&self) -> &Confused {
        &self.confused
    }
    fn confused_mut(&mut self) -> &mut Confused {
        &mut self.confused
    }
}

impl Confused{
    pub fn add_player(game: &mut Game, player: PlayerReference){
        game.confused_mut().players.insert(player);
    }
    pub fn remove_player(game: &mut Game, player: PlayerReference){
        game.confused_mut().players.remove(&player);
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        game.confused().players.contains(&player)
    }
}