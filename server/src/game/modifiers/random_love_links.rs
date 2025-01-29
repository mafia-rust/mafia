use rand::seq::IteratorRandom;

use crate::game::{components::love_linked::LoveLinked, player::PlayerReference, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct RandomLoveLinks;

impl From<&RandomLoveLinks> for ModifierType{
    fn from(_: &RandomLoveLinks) -> Self {
        ModifierType::RandomLoveLinks
    }
}

impl ModifierTrait for RandomLoveLinks{
    fn on_game_start(self, game: &mut Game) {
        for player in PlayerReference::all_players(game) {
            if LoveLinked::get_links(game, player).len() != 0 {continue;}

            let random_unlinked_player = PlayerReference::all_players(game)
                .filter(|p| *p != player)
                .filter(|p| LoveLinked::get_links(game, *p).len()==0)
                .choose(&mut rand::rng());

            if let Some(other_player) = random_unlinked_player {
                LoveLinked::add_love_link(game, player, other_player);
            }else{
                let random_player = PlayerReference::all_players(game)
                    .filter(|p| *p != player)
                    .choose(&mut rand::rng());

                if let Some(other_player) = random_player {
                    LoveLinked::add_love_link(game, player, other_player);
                }
            }
        }
    }
}
