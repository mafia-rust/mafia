use crate::{game::{
    ability_input::*,
    phase::PhaseType, player::PlayerReference, role::Role, Game
}, vec_set};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap {
        if !game.settings.enabled_roles.contains(&Role::Blackmailer) {
            return ControllerParametersMap::default();
        }

        let mut out = ControllerParametersMap::default();

        for player in PlayerReference::all_players(game) {
            out.combine_overwrite(
                ControllerParametersMap::builder(game)
                    .id(ControllerID::forfeit_vote(player))
                    .available_selection(AvailableBooleanSelection)
                    .add_grayed_out_condition(!player.alive(game) || game.current_phase().phase() != PhaseType::Discussion)
                    .reset_on_phase_start(PhaseType::Obituary)
                    .allow_players(vec_set![player])
                    .build_map()
            );
        }

        out
    }

    pub fn on_validated_ability_input_received(game: &mut Game, actor_ref: PlayerReference, input: AbilityInput){
        let Some(selection) = input.get_boolean_selection_if_id(ControllerID::forfeit_vote(actor_ref)) else {return};
        if 
            game.current_phase().phase() == PhaseType::Discussion &&
            actor_ref.alive(game)
        {
            actor_ref.set_forfeit_vote(game, selection.0);
        }
    }
}