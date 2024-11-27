use crate::game::{ability_input::{AbilityID, AbilityInput}, phase::PhaseType, player::PlayerReference, Game};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn on_ability_input_received(game: &mut Game, actor_ref: PlayerReference, input: AbilityInput){
        let Some(selection) = input.get_boolean_selection_if_id(AbilityID::forfeit_vote()) else {return};
        if 
            game.current_phase().phase() == PhaseType::Discussion &&
            actor_ref.alive(game)
        {
            actor_ref.set_forfeit_vote(game, selection.0);
        }
    }
}