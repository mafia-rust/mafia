use crate::game::{player::PlayerReference, verdict::Verdict};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoAbstaining;

/*
    There is modifier specific code in the set_verdict() function
*/
impl From<&NoAbstaining> for ModifierType{
    fn from(_: &NoAbstaining) -> Self {
        ModifierType::NoAbstaining
    }
}
impl ModifierTrait for NoAbstaining{
    fn on_game_start(self, game: &mut crate::game::Game) {
        for player in PlayerReference::all_players(game) {
            player.set_verdict(game, Verdict::Innocent)
        }
    }
}
