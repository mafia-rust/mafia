use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::mimic_win_con::MimicWinCon;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::win_condition::WinCondition;
use crate::game::Game;
use super::RoleState;
use super::{Priority, Role, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Mimic;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Mimic {
    type ClientRoleState = Mimic;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}


        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){

            let target_ref = visit.target;
            
            if target_ref.defense(game).can_block(AttackPower::Basic) || *target_ref.win_condition(game) == WinCondition::RoleStateWon {
                actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                return;
            }
            actor_ref.set_role_state(game, target_ref.role_state(game).clone());
            target_ref.set_role_state(game,RoleState::Mimic(Mimic));
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Mimic, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Mimic, 0),
            false
        )
    }
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        MimicWinCon::new_mimic(game, actor_ref);
        actor_ref.set_win_condition_no_convert_call(game, WinCondition::Mimic(actor_ref));
    }
}
