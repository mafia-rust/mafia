use serde::Serialize;

use crate::game::ability_input::AvailableTwoPlayerOptionSelection;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{
    common_role,
    ControllerID, ControllerParametersMap, GetClientRoleState,
    InsiderGroupID, Role, RoleStateImpl
};


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct MafiaWitch{
    currently_used_player: Option<PlayerReference> 
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

impl RoleStateImpl for MafiaWitch {
    type ClientRoleState = ClientRoleState;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, MafiaWitch{
                currently_used_player: Some(currently_used_player)
            })
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::MafiaWitch, 0))
            .available_selection(AvailableTwoPlayerOptionSelection {
                available_first_players: PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|*p != actor_ref)
                    .filter(|p|!InsiderGroupID::in_same_revealed_group(game, actor_ref, *p))
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
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::MafiaWitch, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, MafiaWitch { currently_used_player: None });
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}
impl GetClientRoleState<ClientRoleState> for MafiaWitch {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}