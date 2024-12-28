use std::collections::HashSet;

use crate::game::{player::PlayerReference, Game};


#[derive(Default)]
pub struct Confused{
    players: HashSet<PlayerReference>
}
impl Confused{
    fn confused<'a>(game: &'a Game)->&'a Self{
        &game.confused
    }
    fn confused_mut<'a>(game: &'a mut Game)->&'a mut Self{
        &mut game.confused
    }


    pub fn add_player(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        confused.players.insert(player);
    }
    pub fn remove_player(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        confused.players.remove(&player);
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        let confused = Self::confused(game);
        confused.players.contains(&player)
    }
}