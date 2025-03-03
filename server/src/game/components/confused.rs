use std::collections::HashMap;

use rand::seq::IteratorRandom;

use crate::{game::{phase::PhaseState, player::PlayerReference, Game}, vec_map::VecMap};

use super::duration::Duration;

#[derive(Default, Clone)]


pub struct ConfusionData{
    duration: Duration,
    red_herrings: VecSet<PlayerReference>,
}
pub struct Confused(VecMap<PlayerReference, (ConfusionData)>);

impl Game {
    fn confused(&self)->&Confused{
        &self.confused
    }
    fn confused_mut(&mut self)->&mut Confused{
        &mut self.confused
    }
}

impl Confused {
    pub fn add_player_permanent(game: &mut Game, player: PlayerReference){
        game.confused_mut().0.insert_unsized(player, Duration::Permanent);
    }

    pub fn add_player_temporary(game: &mut Game, player: PlayerReference, duration: u8){
        game.confused_mut().0.keep_greater(player, Duration::Temporary(duration));
    }

    pub fn remove_player(game: &mut Game, player: PlayerReference){
        game.confused_mut().0.remove(&player);
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        game.confused().0.contains(&player)
    }
    
    /// Decrements confusion durations and removes players whose durations are up
    pub fn on_phase_start(game: &mut Game, phase: PhaseState){
        match phase {
            //feel free to change the phase, right now there aren't any ways to temporarily confuse a player so I chose Night mostly arbitrarily
            PhaseState::Night => {
                game.confused.0.retain_mut(
                    |_, (data)| data.duration.decrement()
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