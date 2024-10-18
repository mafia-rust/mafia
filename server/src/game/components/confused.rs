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
        let intoxicated = Self::confused_mut(game);
        intoxicated.players.insert(player);
    }
    pub fn remove_player(game: &mut Game, player: PlayerReference){
        let intoxicated = Self::confused_mut(game);
        intoxicated.players.remove(&player);
    }

    pub fn is_intoxicated(game: &Game, player: PlayerReference)->bool{
        let intoxicated = Self::confused(game);
        intoxicated.players.contains(&player)
    }
}