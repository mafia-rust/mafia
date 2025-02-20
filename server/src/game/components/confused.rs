use crate::game::{phase::PhaseType, player::PlayerReference, Game};

use super::duration::Duration;

#[derive(Default)]
pub struct Confused {
    players: Vec<PlayerReference>,
    durations: Vec<Duration>,
}


impl Confused{
    fn confused<'a>(game: &'a Game)->&'a Self{
        &game.confused
    }
    fn confused_mut<'a>(game: &'a mut Game)->&'a mut Self{
        &mut game.confused
    }

    pub fn add_player_permanent(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        confused.players.push(player);
        confused.durations.push(Duration::Permanent);
    }

    pub fn add_player(game: &mut Game, player: PlayerReference, duration: u8){
        let confused = Self::confused_mut(game);
        confused.players.push(player);
        confused.durations.push(Duration::Temporary { duration });
    }

    pub fn add_player_from_possession(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        confused.players.push(player);
        confused.durations.push(Duration::Temporary { duration: 0 });
    }

    pub fn remove_player(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        let index = confused.players.iter().position(|x| *x == player);
        match index {
            Some(i) => {
                confused.durations.swap_remove(i); 
                confused.players.swap_remove(i);
                //This is used for cases where a player was confused by multiple sources. 
                Self::remove_player(game, player);
            },
            None => (),
        }
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        let confused = Self::confused(game);
        confused.players.contains(&player)
    }

    /// For abilities that are not affected by the player being possessed.
    /// The reason why there is a distinction is because when a player is possessed by a confused player, then the actions affected
    ///     by the possession should follow the logic they do if the possessed player was confused.
    /// Example: If an Auditor is possessed by a confused Witch, the Auditor should still get the right info because the the info wouldn't 
    ///     be affected by possession.
    pub fn is_confused_not_possess_confused(game: &Game, player:PlayerReference) -> bool{
        //implemented this way because a player can be confused by multiple sources at once.
        let confused = Self::confused(game);
        for i in 0..confused.players.len() {
            if confused.players[i] == player && !confused.durations[i].is_over(){
                return true;
            }
        }
        return false;
    }

    /// Decrements confusion durations and removes players whose durations are up
    pub fn before_phase_end(game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Dusk => {
                let duration_max_index = game.confused.durations.len()-1;
                for i in 0..duration_max_index {
                    let index = duration_max_index - i;
                    let player: PlayerReference = game.drunk_aura.players[i];
                    if player.role(game).any_wildcard() {
                        if game.confused.durations[index].is_over() {
                            game.confused.players.swap_remove(index);
                            game.confused.durations.swap_remove(index);
                        }
                    } else if !game.confused.durations[index].decrement() {
                        game.confused.players.swap_remove(index);
                        game.confused.durations.swap_remove(index);
                    }
                }
            },
            PhaseType::Night => {
                let duration_max_index = game.confused.durations.len()-1;
                for i in 0..duration_max_index {
                    let index = duration_max_index - i;
                    if game.confused.durations[index].is_over() {
                        game.confused.players.swap_remove(index);
                        game.confused.durations.swap_remove(index);
                    }
                }
            },
            _=>{}
        }
    }
}