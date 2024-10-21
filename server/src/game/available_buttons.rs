use serde::Serialize;

use super::{modifiers::{ModifierType, Modifiers}, phase::PhaseType, player::PlayerReference, Game};


#[derive(Debug, Clone, Serialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableButtons{
    pub vote: bool,
    pub target: bool,
    pub day_target: bool,
}
impl AvailableButtons{
    pub fn from_player_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference)->Self{
        Self{
            vote: 
                !Modifiers::modifier_is_enabled(game, ModifierType::NoTrials) &&
                actor_ref != target_ref &&
                game.current_phase().phase() == PhaseType::Nomination &&
                actor_ref.chosen_vote(game).is_none() && 
                !actor_ref.forfeit_vote(game) &&
                actor_ref.alive(game) && target_ref.alive(game),

            target: 
                actor_ref.can_select(game, target_ref) && 
                game.current_phase().is_night(),

            day_target: 
                actor_ref.can_day_target(game, target_ref)
        }
    }
    pub fn from_player(game: &Game, actor_ref: PlayerReference)->Vec<Self>{
        let mut out = Vec::new();

        for target_ref in PlayerReference::all_players(game){
            out.push(Self::from_player_target(game, actor_ref, target_ref));
        }
        out
    }
}


