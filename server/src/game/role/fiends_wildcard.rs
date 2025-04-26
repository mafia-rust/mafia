use serde::{Serialize, Deserialize};

use crate::game::ability_input::{AbilityInput, AvailableRoleOptionSelection, RoleOptionSelection};
use crate::game::{attack_power::DefensePower, role_list::RoleSet};
use crate::game::player::PlayerReference;
use crate::game::Game;

use super::wild_card::Wildcard;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FiendsWildcard;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for FiendsWildcard {
    type ClientRoleState = FiendsWildcard;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, _input_player: PlayerReference, ability_input: AbilityInput) {
        let Some(RoleOptionSelection(Some(role))) = ability_input.get_role_option_selection_if_id(ControllerID::role(
            actor_ref, 
            Role::FiendsWildcard, 
            0
        )) else {return};
        Wildcard::become_role(game, actor_ref, role);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::FiendsWildcard, 0))
            .single_role_selection_typical(game, |role|*role != Role::FiendsWildcard)
            .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
            .allow_players([actor_ref])
            .build_map()
    }
}