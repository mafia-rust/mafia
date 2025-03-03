use serde::Serialize;

use crate::game::{components::vampire_tracker::VampireTracker, phase::PhaseType};
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set::VecSet;
use super::{Role, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
// Most of the implementation is handled in vampire_tracker
pub struct Vampire;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Vampire {
    type ClientRoleState = Vampire;
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        // crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
        //     game,
        //     actor_ref,
        //     false,
        //     true,
        //     game.day_number() <= 1,
        //     ControllerID::role(actor_ref, Role::Vampire, 0)
        // )
        ControllerParametersMap::new_controller_fast(
            game, 
            ControllerID::role(actor_ref, Role::Vampire, 0), 
            AvailableAbilitySelection::new_player_list(PlayerReference::all_players(game)
                    .filter(|p|
                        p.alive(game) &&
                        !VampireTracker::is_tracked(game, *p)
                    )
                    .collect(),
                    false,
                    Some(1)
                ),
            AbilitySelection::new_player_list(vec![]), 
            false, 
            Some(PhaseType::Discussion), 
            false, 
            VecSet::with_first(actor_ref)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Vampire, 0),
            true
        )
    }
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        VampireTracker::before_initial_role_creation(game, actor_ref);
    }
}
