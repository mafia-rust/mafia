use serde::Serialize;

use crate::game::ability_input::{AvailableIntegerSelection, IntegerSelection, PlayerListSelection};
use crate::game::attack_power::AttackPower;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::grave::GraveKiller;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Jack;

impl RoleStateImpl for Jack {
    type ClientRoleState = Jack;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        let selection = Self::ability_type_selection(game, actor_ref);
        match (priority, selection) {
            (OnMidnightPriority::Investigative, JackAbilityType::Investigate) => {
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
            (OnMidnightPriority::Heal, JackAbilityType::Protect) => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target) = actor_visits.first().map(|v|v.target) else {return};
                if target.all_night_visits_cloned(game).into_iter().any(|v|v.target == actor_ref) {
                    actor_ref.guard_player(game, midnight_variables, target);
                }
            },
            (OnMidnightPriority::Kill, JackAbilityType::Kill) => {
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
        
        let mut ctrl = ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Jack, 0))
            .available_selection(AvailableIntegerSelection {
                min: 0,
                #[expect(clippy::cast_possible_wrap, clippy::arithmetic_side_effects, reason = "clamped")]
                max: 1 + game.day_number().clamp(1, 2) as i8
            })
            .default_selection(selection.into())
            .allow_players([actor_ref])
            .night_typical(actor_ref)
            .add_grayed_out_condition(false)
            .build_map();

        match selection {
            JackAbilityType::None => (),
            JackAbilityType::Investigate => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Jack, 1))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            JackAbilityType::Protect => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Jack, 2))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            JackAbilityType::Kill => {
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
        }
        ctrl
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let type_selection = Self::ability_type_selection(game, actor_ref);
        match type_selection {
            JackAbilityType::None => Vec::new(),
            JackAbilityType::Investigate => crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::Jack, 1),
                    false
                ),
            JackAbilityType::Protect => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Jack, 2),
                false
            ),
            JackAbilityType::Kill => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Jack, 4),
                false
            ),
        }
    }
}
impl Jack {
    fn ability_type_selection(game: &Game, actor_ref: PlayerReference) -> JackAbilityType {
        ControllerID::role(actor_ref, Role::Jack, 0)
            .get_integer_selection(game)
            .cloned()
            .unwrap_or_default()
            .into()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
enum JackAbilityType {
    #[default]
    None,
    Investigate,
    Protect,
    Kill,
}

impl From<i8> for JackAbilityType {
    fn from(v: i8) -> JackAbilityType {
        match v {
            0 => JackAbilityType::None,
            1 => JackAbilityType::Investigate,
            2 => JackAbilityType::Protect,
            3 => JackAbilityType::Kill,
            _ => JackAbilityType::None
        }
    }
}
impl From<JackAbilityType> for i8 {
    fn from(v: JackAbilityType) -> i8 {
        match v {
            JackAbilityType::None => 0,
            JackAbilityType::Investigate => 1,
            JackAbilityType::Protect => 2,
            JackAbilityType::Kill => 3,
        }
    }
}
impl From<JackAbilityType> for IntegerSelection {
    fn from(v: JackAbilityType) -> IntegerSelection {
        IntegerSelection(v.into())
    }
}
impl From<IntegerSelection> for JackAbilityType {
    fn from(v: IntegerSelection) -> JackAbilityType {
        v.0.into()
    }
}