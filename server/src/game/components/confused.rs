use rand::seq::IteratorRandom;

use crate::{game::{phase::PhaseState, player::PlayerReference, Game}, vec_map::VecMap};

use super::duration::Duration;

#[derive(Default, Clone)]
pub struct Confused {
    pub players_durations: VecMap<PlayerReference, Duration>,
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
        confused.players_durations.insert(player, Duration::Permanent);
    }

    pub fn add_player_temporary(game: &mut Game, player: PlayerReference, duration: u8){
        let confused = Self::confused_mut(game);
        confused.players_durations.keep_greater(player, Duration::Temporary(duration));
    }

    pub fn remove_player(game: &mut Game, player: PlayerReference){
        let confused = Self::confused_mut(game);
        confused.players_durations.remove(&player);
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        let confused = Self::confused(game);
        confused.players_durations.contains(&player)
    }
    
    /// Decrements confusion durations and removes players whose durations are up
    pub fn on_phase_start(game: &mut Game, phase: PhaseState){
        match phase {
            //feel free to change the phase, right now there aren't any ways to temporarily confuse a player so I chose Night mostly arbitrarily
            PhaseState::Night => {
                game.confused.players_durations.retain_mut(
                    |_, duration| duration.decrement()
                );
            },
            _=>{}
        }
    }
}

impl PlayerReference {
    pub fn generate_red_herring(self, game: &Game) -> Option<PlayerReference>{
        return PlayerReference::all_players(game)
            .filter(|player|
                player.alive(game) &&
                *player != self
            )
            .choose(&mut rand::rng())
    }
}