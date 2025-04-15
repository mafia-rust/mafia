use serde::{Serialize, Deserialize};

use crate::game::ability_input::AvailableRoleOptionSelection;
use crate::game::{attack_power::DefensePower, role_list::RoleSet};
use crate::game::phase::PhaseType;
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
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                if actor_ref.ability_deactivated_from_death(game) {return;}
                Wildcard::become_role(game, actor_ref, Role::FiendsWildcard);
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::FiendsWildcard, 0))
            .available_selection(AvailableRoleOptionSelection(
                RoleSet::Fiends.get_roles().into_iter().filter(|role|
                    game.settings.enabled_roles.contains(role) && *role != Role::FiendsWildcard
                ).map(Some).chain(std::iter::once(None)).collect()
            ))
            .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
            .allow_players([actor_ref])
            .build_map()
    }
}