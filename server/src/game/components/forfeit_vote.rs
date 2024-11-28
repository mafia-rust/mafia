use crate::game::{ability_input::{ability_selection::AvailableAbilitySelection, AbilityID, AbilityInput, AvailableAbilityInput}, phase::PhaseType, player::PlayerReference, role::Role, Game};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn available_ability_input(game: &Game, actor_ref: PlayerReference)->AvailableAbilityInput {
        if 
            actor_ref.alive(game) &&
            game.current_phase().phase() == PhaseType::Discussion &&
            game.settings.enabled_roles.contains(&Role::Blackmailer) 
        {
            AvailableAbilityInput::new_ability(
                AbilityID::ForfeitVote,
                AvailableAbilitySelection::Boolean
            )
        } else {
            AvailableAbilityInput::default()
        }
    }

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