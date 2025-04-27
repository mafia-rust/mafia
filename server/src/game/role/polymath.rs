use serde::Serialize;

use crate::game::ability_input::{AvailableIntegerSelection, AvailableTwoPlayerOptionSelection, IntegerSelection, PlayerListSelection};
use crate::game::attack_power::AttackPower;
use crate::game::components::transport::{Transport, TransportPriority};
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::grave::GraveKiller;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_map;

use super::detective::Detective;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Polymath;

impl RoleStateImpl for Polymath {
    type ClientRoleState = Polymath;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        let selection = Self::ability_type_selection(game, actor_ref);
        match (priority, selection) {
            (OnMidnightPriority::Investigative, PolymathAbilityType::Investigate) => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target) = actor_visits.first().map(|v|v.target) else {return};
                actor_ref.push_night_message(midnight_variables, 
                    ChatMessageVariant::PolymathSnoopResult {inno:
                        actor_ref.all_night_visitors_cloned(game).is_empty() &&
                        !Detective::player_is_suspicious(game, midnight_variables, target)
                    }
                );
            },
            (OnMidnightPriority::Heal, PolymathAbilityType::Protect) => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target) = actor_visits.first().map(|v|v.target) else {return};
                if target.all_night_visits_cloned(game).into_iter().any(|v|v.target == actor_ref) {
                    actor_ref.guard_player(game, midnight_variables, target);
                }
            },
            (OnMidnightPriority::Warper, PolymathAbilityType::Support) => {    
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(from) = actor_visits.first().map(|v| v.target) else {return};
                let Some(to) = actor_visits.get(1).map(|v| v.target) else {return};
                
                Transport::transport(
                    game, 
                    midnight_variables, 
                    TransportPriority::Warper, 
                    &vec_map![(from, to)],
                    |_| true,
                    false,
                );
            }
            (OnMidnightPriority::Kill, PolymathAbilityType::Kill) => {
                let Some(&actor_visit) = actor_ref.untagged_night_visits_cloned(game).first() else {return};
                let Some(PlayerListSelection(mark)) = ControllerID::role(actor_ref, Role::Polymath, 4)
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
            .id(ControllerID::role(actor_ref, Role::Polymath, 0))
            .available_selection(AvailableIntegerSelection {
                min: 0,
                #[expect(clippy::cast_possible_wrap, clippy::arithmetic_side_effects, reason = "clamped")]
                max: 2 + game.day_number().clamp(1, 2) as i8
            })
            .default_selection(selection.into())
            .allow_players([actor_ref])
            .night_typical(actor_ref)
            .add_grayed_out_condition(false)
            .build_map();

        match selection {
            PolymathAbilityType::None => (),
            PolymathAbilityType::Investigate => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Polymath, 1))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            PolymathAbilityType::Protect => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Polymath, 2))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            PolymathAbilityType::Support => ctrl.combine_overwrite(
                ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Polymath, 3))
                .available_selection(AvailableTwoPlayerOptionSelection::same_players(
                    PlayerReference::all_players(game)
                        .filter(|p|p.alive(game))
                        .collect(), 
                    false, 
                    true
                )).night_typical(actor_ref)
                .add_grayed_out_condition(false)
                .build_map()
            ),
            PolymathAbilityType::Kill => {
                ctrl.combine_overwrite( //
                    ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Polymath, 4))
                    .single_player_selection_typical(actor_ref, false, true)
                    .night_typical(actor_ref)
                    .add_grayed_out_condition(game.day_number() == 1)
                    .build_map()
                );
                ctrl.combine_overwrite( //
                    ControllerParametersMap::builder(game)
                    .id(ControllerID::role(actor_ref, Role::Polymath, 5))
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
            PolymathAbilityType::None => Vec::new(),
            PolymathAbilityType::Investigate => crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::Polymath, 1),
                    false
                ),
            PolymathAbilityType::Protect => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Polymath, 2),
                false
            ),
            PolymathAbilityType::Support => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Polymath, 3),
                false
            ),
            PolymathAbilityType::Kill => crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Polymath, 5),
                false
            ),
        }
    }
}
impl Polymath {
    fn ability_type_selection(game: &Game, actor_ref: PlayerReference) -> PolymathAbilityType {
        ControllerID::role(actor_ref, Role::Polymath, 0)
            .get_integer_selection(game)
            .cloned()
            .unwrap_or_default()
            .into()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
enum PolymathAbilityType {
    #[default]
    None,
    Investigate,
    Protect,
    Support,
    Kill,
}

impl From<PolymathAbilityType> for i8 {
    fn from(v: PolymathAbilityType) -> i8 {
        match v {
            PolymathAbilityType::None => 0,
            PolymathAbilityType::Investigate => 1,
            PolymathAbilityType::Protect => 2,
            PolymathAbilityType::Support => 3,
            PolymathAbilityType::Kill => 4,
        }
    }
}
impl From<i8> for PolymathAbilityType {
    fn from(v: i8) -> PolymathAbilityType {
        match v {
            0 => PolymathAbilityType::None,
            1 => PolymathAbilityType::Investigate,
            2 => PolymathAbilityType::Protect,
            3 => PolymathAbilityType::Support,
            4 => PolymathAbilityType::Kill,
            _ => PolymathAbilityType::None
        }
    }
}
impl From<PolymathAbilityType> for IntegerSelection {
    fn from(v: PolymathAbilityType) -> IntegerSelection {
        IntegerSelection(v.into())
    }
}
impl From<IntegerSelection> for PolymathAbilityType {
    fn from(v: IntegerSelection) -> PolymathAbilityType {
        v.0.into()
    }
}