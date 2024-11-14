use crate::game::{ability_input::AbilityInput, phase::PhaseType, player::PlayerReference, Game};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn on_ability_input_received(game: &mut Game, actor_ref: PlayerReference, input: AbilityInput){
        let AbilityInput::ForfeitVote{selection} = input else {return};
        if 
            game.current_phase().phase() == PhaseType::Discussion &&
            actor_ref.alive(game)
        {
            actor_ref.set_forfeit_vote(game, selection.0);
        }
    }
}