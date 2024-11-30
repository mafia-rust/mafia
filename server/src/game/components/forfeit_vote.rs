use crate::game::{
    ability_input::*,
    phase::PhaseType, player::PlayerReference, role::Role, Game
};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn available_ability_input(game: &Game, actor_ref: PlayerReference)->AvailableAbilitiesData {
        if !game.settings.enabled_roles.contains(&Role::Blackmailer) {
            return AvailableAbilitiesData::default();
        }

        AvailableAbilitiesData::new_ability_fast(
            game,
            AbilityID::forfeit_vote(),
            AvailableAbilitySelection::new_boolean(),
            AbilitySelection::new_boolean(false),
            !actor_ref.alive(game) || game.current_phase().phase() != PhaseType::Discussion,
            Some(PhaseType::Obituary),
            false
        )
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