use crate::game::{
    ability_input::{ability_id::AbilityID, ability_selection::{AbilitySelection, AvailableAbilitySelection}, available_abilities_data::{AvailableAbilitiesData, AvailableSingleAbilityData}, selection_type::BooleanSelection, AbilityInput},
    phase::PhaseType, player::PlayerReference, role::Role, Game
};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn available_ability_input(game: &Game, actor_ref: PlayerReference)->AvailableAbilitiesData {
        if !game.settings.enabled_roles.contains(&Role::Blackmailer) {
            return AvailableAbilitiesData::default();
        }
        
        if let Some(mut ability) = AvailableSingleAbilityData::new_obituary_resetting_default_and_available(
            game,
            AbilitySelection::Boolean{selection: BooleanSelection(false)},
            AvailableAbilitySelection::Boolean
        ){
            if !(
                actor_ref.alive(game) &&
                game.current_phase().phase() == PhaseType::Discussion
            ) {
                ability.set_grayed_out(true); 
            }
            AvailableAbilitiesData::new_ability(AbilityID::ForfeitVote, ability)
        }else{
            AvailableAbilitiesData::default()
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