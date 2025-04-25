use serde::Serialize;

use crate::game::ability_input::{AvailableRoleOptionSelection, PlayerListSelection, RoleOptionSelection};
use crate::game::attack_power::AttackPower;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::grave::GraveKiller;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Jack;

impl RoleStateImpl for Jack {
    type ClientRoleState = Jack;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        let RoleOptionSelection(Some(role)) = Self::ability_type_selection(game, actor_ref) else {return};
        match (priority, role) {
            (OnMidnightPriority::Investigative, Role::Snoop) => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let inno = actor_ref.all_night_visitors_cloned(game).is_empty() &&
                    !visit.target.has_suspicious_aura(game, midnight_variables) && 
                    (
                        visit.target.win_condition(game).friends_with_resolution_state(GameConclusion::Town) ||
                        visit.target.has_innocent_aura(game)
                    );
                actor_ref.push_night_message(midnight_variables, 
                    ChatMessageVariant::JackSnoopResult { inno }
                );
            },
            (OnMidnightPriority::Heal, Role::Armorsmith) => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target) = actor_visits.first().map(|v|v.target) else {return};
                if target.all_night_visits_cloned(game).into_iter().any(|v|v.target == actor_ref) {
                    actor_ref.guard_player(game, midnight_variables, target);
                }
            },
            (OnMidnightPriority::Kill, Role::Marksman) => {
                let Some(&actor_visit) = actor_ref.untagged_night_visits_cloned(game).first() else {return};
                let Some(PlayerListSelection(mark)) = ControllerID::role(actor_ref, Role::Jack, 3)
                    .get_player_list_selection(game)
                    .cloned() else {return};
                let Some(mark) = mark.first() else {return};
                if !actor_visit.target.all_night_visitors_cloned(game).contains(mark) {return};
                
                mark.try_night_kill_single_attacker (
                    actor_ref,
                    game, 
                    midnight_variables, 
                    GraveKiller::Role(Role::Marksman), 
                    AttackPower::Basic, 
                    false
                );
            },
            _=>(),
        }


        
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let selection = Self::ability_type_selection(game, actor_ref);
        
        let mut ctrl = Self::ability_type_ctrl(game, actor_ref, selection.clone());

        match selection.0 {
            None => (),
            Some(Role::Snoop) => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Jack, 1))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            Some(Role::Armorsmith) => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Jack, 2))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            Some(Role::Marksman) => {
                ctrl.combine_overwrite( //
                    ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Jack, 3))
                    .single_player_selection_typical(actor_ref, false, true)
                    .night_typical(actor_ref)
                    .add_grayed_out_condition(game.day_number() == 1)
                    .build_map()
                );
                ctrl.combine_overwrite( //
                    ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Jack, 4))
                    .single_player_selection_typical(actor_ref, true, true)
                    .night_typical(actor_ref)
                    .add_grayed_out_condition(game.day_number() == 1)
                    .build_map()
                );
            },
            Some(_) => (),
        }
        ctrl
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let type_selection = Self::ability_type_selection(game, actor_ref);
        match type_selection.0 {
            None => Vec::new(),
            Some(Role::Snoop) => crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::Jack, 1),
                    false
                ),
            Some(Role::Armorsmith) => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Jack, 2),
                false
            ),
            Some(Role::Marksman) => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Jack, 4),
                false
            ),
            Some(_) => Vec::new(),
        }
    }
}
impl Jack {
    fn ability_type_ctrl(game: &Game, actor_ref: PlayerReference, default: RoleOptionSelection) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Jack, 0))
            .available_selection(AvailableRoleOptionSelection(
                if game.day_number() == 1 {
                    vec_set![None, Some(Role::Snoop), Some(Role::Armorsmith)]
                } else {
                    vec_set![None, Some(Role::Snoop), Some(Role::Armorsmith), Some(Role::Marksman)]
                }
            )).default_selection(default)
            .night_typical(actor_ref)
            .add_grayed_out_condition(false)
            .build_map()
    }
    fn ability_type_selection(game: &Game, actor_ref: PlayerReference) -> &RoleOptionSelection {
        ControllerID::role(actor_ref, Role::Jack, 0).get_role_option_selection(game).unwrap_or(&RoleOptionSelection(None))
    }
}