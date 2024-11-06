use std::collections::HashSet;

use crate::game::{player::PlayerReference, Game};

use super::confused::Confused;

#[derive(Default, Clone)]
pub struct DrunkAura {
    pub players: HashSet<PlayerReference>,
}

impl DrunkAura {
    fn drunk_aura(game: &Game) -> &Self {
        &game.drunk_aura
    }
    fn drunk_aura_mut(game: &mut Game) -> &mut Self {
        &mut game.drunk_aura
    }

    pub fn add_player(game: &mut Game, player: PlayerReference) {
        Self::drunk_aura_mut(game).players.insert(player);
    }
    pub fn remove_player(game: &mut Game, player: PlayerReference) {
        Self::drunk_aura_mut(game).players.remove(&player);
    }

    pub fn has_drunk_aura(game: &Game, player: PlayerReference) -> bool {
        Self::drunk_aura(game).players.contains(&player)
    }

    pub fn on_role_switch(game: &mut Game, player: PlayerReference) {
        if Self::has_drunk_aura(game, player) {
            Self::remove_player(game, player);
            Confused::remove_player(game, player);
        }
    }
}