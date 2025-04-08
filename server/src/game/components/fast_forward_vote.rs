use crate::game::{ability_input::{AbilityInput, AvailableBooleanSelection, ControllerID, ControllerParametersMap}, event::on_fast_forward::OnFastForward, player::PlayerReference, Game};


pub struct FastForwardVote;

impl FastForwardVote {
    pub fn controller_parameters_map(game: &Game) -> ControllerParametersMap {
        ControllerParametersMap::combine(
            PlayerReference::all_players(game)
                .map(|player| {
                    ControllerParametersMap::builder(game)
                        .id(ControllerID::fast_forward_vote(player))
                        .available_selection(AvailableBooleanSelection)
                        .add_grayed_out_condition(!player.alive(game))
                        .reset_on_all_phases()
                        .allow_players([player])
                        .build_map()
                }),
        )
    }
    pub fn on_validated_ability_input_received(game: &mut Game, actor_ref: PlayerReference, input: AbilityInput) {
        if input.id() != ControllerID::fast_forward_vote(actor_ref) {
            return;
        }
        if game.phase_machine.time_remaining.is_some_and(|d|!d.is_zero()) 
            && PlayerReference::all_players(game)
                .filter(|p| p.alive(game) && !p.is_disconnected(game))
                .all(|p| game.saved_controllers
                    .get_controller_current_selection_boolean(ControllerID::fast_forward_vote(p))
                    .is_some_and(|c| c.0))
        {
            OnFastForward::invoke(game);
        }
    }
}