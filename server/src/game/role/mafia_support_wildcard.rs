use serde::{Serialize, Deserialize};

use crate::game::{ability_input::AbilityInput, role_list::RoleSet};
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;
use crate::game::Game;

use super::{wild_card::Wildcard, ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MafiaSupportWildcard;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for MafiaSupportWildcard {
    type ClientRoleState = MafiaSupportWildcard;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, _: AbilityInput) {
        if input_player == actor_ref {
            Wildcard::become_role(game, actor_ref, Role::MafiaSupportWildcard);
        }
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::MafiaSupportWildcard, 0))
            .single_role_selection_role_set(game, RoleSet::MafiaSupport)
            .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
            .allow_players([actor_ref])
            .build_map()
    }
}