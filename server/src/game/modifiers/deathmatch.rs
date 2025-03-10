use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::{PhaseState, PhaseType}, player::PlayerReference, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Deathmatch;

/*
    There is modifier specific code in server.src\game\role\mod.rs
    in the defense function for role
*/
impl From<&Deathmatch> for ModifierType{
    fn from(_: &Deathmatch) -> Self {
        ModifierType::Deathmatch
    }
}

impl ModifierTrait for Deathmatch {
    fn on_game_start(self, game: &mut Game) {
        for player in PlayerReference::all_players(game){
            game.add_message_to_chat_group(
                ChatGroup::All, 
                ChatMessageVariant::PlayerHasWinCondition{player: player.index(), win_condition: player.win_condition(game).clone()}
            );
        }
    }
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        if phase.phase() == PhaseType::Nomination {
            game.on_fast_forward();
        }
    }
}