use crate::game::{
    ability_input::*,
    phase::PhaseType, player::PlayerReference, role::Role, Game
};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn available_abilities(game: &Game)->AllPlayersAvailableAbilities {
        if !game.settings.enabled_roles.contains(&Role::Blackmailer) {
            return AllPlayersAvailableAbilities::default();
        }

        let mut out = AllPlayersAvailableAbilities::default();

        for player in PlayerReference::all_players(game) {
            out.combine_overwrite(
                AllPlayersAvailableAbilities::new_ability_fast(
                    game,
                    player,
                    AbilityID::forfeit_vote(),
                    AvailableAbilitySelection::new_boolean(),
                    AbilitySelection::new_boolean(false),
                    !player.alive(game) || game.current_phase().phase() != PhaseType::Discussion,
                    Some(PhaseType::Obituary),
                    false
                )
            );
        }

        out
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