use std::collections::HashSet;

use crate::game::{player::PlayerReference, Game};

use super::confused::Confused;

#[derive(Default, Clone)]
pub struct DrunkAura {
    pub players: HashSet<PlayerReference>,
}

impl Game {
    fn drunk_aura(&self) -> &DrunkAura {
        &self.drunk_aura
    }
    fn drunk_aura_mut(&mut self) -> &mut DrunkAura {
        &mut self.drunk_aura
    }
}

impl DrunkAura {
    pub fn add_player(game: &mut Game, player: PlayerReference) {
        game.drunk_aura_mut().players.insert(player);
    }
    pub fn remove_player(game: &mut Game, player: PlayerReference) {
        game.drunk_aura_mut().players.remove(&player);
    }

    pub fn has_drunk_aura(game: &Game, player: PlayerReference) -> bool {
        game.drunk_aura().players.contains(&player)
    }

    pub fn on_role_switch(game: &mut Game, player: PlayerReference) {
        if Self::has_drunk_aura(game, player) {
            Self::remove_player(game, player);
            Confused::remove_player(game, player);
        }
    }
}