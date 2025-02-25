use crate::{game::{phase::PhaseState, player::PlayerReference, Game}, vec_map::VecMap};

use super::{duration::Duration, confused::Confused};

#[derive(Default, Clone)]
pub struct DrunkAura {
    pub players_durations: VecMap<PlayerReference, Duration>,
}

impl DrunkAura {
    fn drunk_aura(game: &Game) -> &Self {
        &game.drunk_aura
    }
    fn drunk_aura_mut(game: &mut Game) -> &mut Self {
        &mut game.drunk_aura
    }

    pub fn add_player_permanent(game: &mut Game, player: PlayerReference){
        let drunk_aura = Self::drunk_aura_mut(game);
        drunk_aura.players_durations.insert(player, Duration::Permanent);
    }

    pub fn add_player_temporary(game: &mut Game, player: PlayerReference, duration: u8){
        let drunk_aura = Self::drunk_aura_mut(game);
        drunk_aura.players_durations.keep_greater(player, Duration::Temporary(duration));
    }

    pub fn remove_player(game: &mut Game, player: PlayerReference){
        let drunk_aura = Self::drunk_aura_mut(game);
        drunk_aura.players_durations.remove(&player);
    }
  
    pub fn has_drunk_aura(game: &Game, player: PlayerReference) -> bool {
        let drunk_aura = Self::drunk_aura(game);
        drunk_aura.players_durations.contains(&player)
    }

    pub fn on_role_switch(game: &mut Game, player: PlayerReference) {
        Self::remove_player(game, player);
        Confused::remove_player(game, player);
    }

    ///Decrements drunk aura durations and removes players whose durations are up
    pub fn on_phase_start(game: &mut Game, phase: PhaseState){
        match phase {
            //feel free to change the phase, right now there aren't any ways to temporarily give a player drunk aura so I chose Night mostly arbitrarily
            PhaseState::Night => {
                game.drunk_aura.players_durations.retain_mut(
                    |_, duration| duration.decrement()
                );
            },
            _=>{}
        }
    }
}
