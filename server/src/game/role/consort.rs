use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::PlayerReference};


use crate::game::visit::Visit;
use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Consort;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Consort {
    type ClientRoleState = Consort;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Roleblock {return;}
        

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;

            target_ref.roleblock(game, true);
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            false,
            ControllerID::role(actor_ref, Role::Consort, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Consort, 0),
            false
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}
