use serde::Serialize;

use crate::game::ability_input::AvailableTwoPlayerOptionSelection;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::win_condition::WinCondition;
use crate::game::attack_power::DefensePower;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{
    common_role,
    ControllerID,
    ControllerParametersMap, GetClientRoleState,
    Role, RoleStateImpl
};


#[derive(Clone, Debug, Default)]
pub struct Witch{
    currently_used_player: Option<PlayerReference> 
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Witch {
    type ClientRoleState = ClientRoleState;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, midnight_variables, priority, self.currently_used_player){
            actor_ref.set_role_state(game, Witch{
                currently_used_player: Some(currently_used_player)
            })
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Witch, 0))
            .available_selection(AvailableTwoPlayerOptionSelection {
                available_first_players: PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|*p != actor_ref)
                    .collect(),
                available_second_players: PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .collect(),
                can_choose_duplicates: true,
                can_choose_none: true
            })
            .night_typical(actor_ref)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits_possession(
            game, actor_ref, ControllerID::role(actor_ref, Role::Witch, 0)
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    WinCondition::are_friends(p.win_condition(game), actor_ref.win_condition(game))
                )

        {
            actor_ref.leave_town(game);
        }
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, Witch { currently_used_player: None });
        }
    }
    fn on_player_roleblocked(self, _game: &mut Game, _midnight_variables: &mut MidnightVariables, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}
impl GetClientRoleState<ClientRoleState> for Witch {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}