use serde::Serialize;

use crate::game::ability_input::AbilitySelection;
use crate::game::components::detained::Detained;
use crate::game::phase::PhaseType;
use crate::game::{ability_input::AvailableAbilitySelection, attack_power::DefensePower};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{common_role, ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Serialize, Debug, Default)]
pub struct Wanderer;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Wanderer {
    type ClientRoleState = Wanderer;
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let grayed_out = actor_ref.ability_deactivated_from_death(game) ||
            Detained::is_detained(game, actor_ref);

            common_role::controller_parameters_map_player_list_night_typical(
                game, 
                actor_ref, 
                true, 
                true, 
                false,
                ControllerID::Role { player: actor_ref, role: Role::Wanderer, id: 0 }
                //when the remove disabled roles PR gets merged, swap its function for this
            ).combine_overwrite_owned(ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::Role { player: actor_ref, role: Role::Wanderer, id: 1 },
                AvailableAbilitySelection::new_role_option(
                    game.settings.enabled_roles.iter()
                    .map(|r|Some(*r))
                    .chain(None)
                    .collect()
                ),
                AbilitySelection::new_player_list(Vec::new()),
                grayed_out,
                Some(PhaseType::Obituary),
                false,
                vec_set!(actor_ref)
            )).combine_overwrite_owned(
            common_role::controller_parameters_map_role_outline_typical(
                game, 
                actor_ref, 
                true, 
                false, 
                ControllerID::Role { player: actor_ref, role: Role::Wanderer, id: 2 }
            )
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let mut visits = common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Wanderer, 0),
            false
        );
        visits.append(&mut common_role::convert_controller_selection_to_visits(
            game, 
            actor_ref, 
            ControllerID::role(actor_ref, Role::Wanderer, 1), 
            false
        ));
        visits.append(&mut common_role::convert_controller_selection_to_visits(
            game, 
            actor_ref, 
            ControllerID::role(actor_ref, Role::Wanderer, 2), 
            false
        ));
        return visits
    }
}