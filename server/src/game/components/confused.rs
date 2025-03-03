use rand::seq::IteratorRandom;

use crate::{game::{phase::PhaseState, player::PlayerReference, Game}, vec_map::VecMap};

use super::status_duration::StatusDuration;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd)]
pub struct ConfusionData{
    pub duration: StatusDuration,
    pub red_herrings: Vec<PlayerReference>,
}
impl ConfusionData {
    pub fn new_perm(game: &Game, player: PlayerReference) -> ConfusionData {
        ConfusionData {
            duration: StatusDuration::Permanent,
            red_herrings: Self::generate_red_herrings(game,player),
        }
    }
    pub fn new_temp(game: &Game, player: PlayerReference, duration: u8) -> ConfusionData {
        ConfusionData {
            duration: StatusDuration::Temporary(duration),
            red_herrings: Self::generate_red_herrings(game,player),
        }
    }
    pub fn generate_red_herrings(game: &Game, player: PlayerReference) -> Vec<PlayerReference> {
        let count = game.assignments.iter()
        .filter(|a|a.2.is_evil())
        .count();
        PlayerReference::all_players(game)
                .filter(|p|*p != player)
                .choose_multiple(&mut rand::rng(), count)
        
    }
}
impl Ord for ConfusionData{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.duration.cmp(&other.duration)
    }
}


#[derive(Default, Clone)]
pub struct Confused(pub VecMap<PlayerReference, ConfusionData>);

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
        let data = ConfusionData::new_perm(game, player);
        game.confused_mut().0.insert_unsized(player, data);
    }

    pub fn add_player_temporary(game: &mut Game, player: PlayerReference, duration: u8){
        let data = ConfusionData::new_temp(game, player, duration);
        game.confused_mut().0.keep_greater_unsized(player, data);
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
                    |_, data| data.duration.decrement()
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