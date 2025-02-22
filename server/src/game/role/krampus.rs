use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::Grave;
use crate::game::phase::PhaseType;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Krampus {
    ability: KrampusAbility,
    last_used_ability: Option<KrampusAbility>
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum KrampusAbility {
    #[default] DoNothing,
    Kill
}

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Krampus {
    type ClientRoleState = ();
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);

        match (priority, self.ability) {
            (Priority::Kill, KrampusAbility::Kill) => {
                if let Some(visit) = actor_visits.first() {
                    let target_ref = visit.target;

                    target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Krampus), AttackPower::Basic, true);

                    actor_ref.set_role_state(game, Krampus {
                        last_used_ability: Some(KrampusAbility::Kill),
                        ..self
                    });
                }
            }
            (Priority::Investigative, _) => {
                if let Some(visit) = actor_visits.first() {
                    let target_ref = visit.target;

                    actor_ref.push_night_message(game, 
                        ChatMessageVariant::TargetHasRole { role: target_ref.role(game) }
                    );
                    actor_ref.push_night_message(game, 
                        ChatMessageVariant::TargetHasWinCondition { win_condition: target_ref.win_condition(game).clone() }
                    );
                }
            }
            _ => {}
        }

        if self.ability == KrampusAbility::DoNothing {
            actor_ref.set_role_state(game, Krampus {
                last_used_ability: Some(KrampusAbility::DoNothing),
                ..self
            });
        }
    }
    fn on_any_death(self, game: &mut Game, _actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        // Prevent stack overflow, but also if either of these people die there can't be a stalemate anyway
        if matches!(dead_player_ref.role(game), Role::Krampus | Role::SantaClaus) { return };

        Self::check_for_stalemate(game);
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        // There's no on_win_condition_changed, and I'm not gonna make one.
        Self::check_for_stalemate(game);

        if phase == PhaseType::Obituary || phase == PhaseType::Night {
            let mut new_state = self.clone();
            
            if let Some(ability) = self.last_used_ability {
                new_state.ability = match ability {
                    KrampusAbility::Kill => KrampusAbility::DoNothing,
                    KrampusAbility::DoNothing => KrampusAbility::Kill,
                };
                new_state.last_used_ability = None;
            }

            actor_ref.add_private_chat_message(game, ChatMessageVariant::NextKrampusAbility { ability: new_state.ability });
            actor_ref.set_role_state(game, new_state);
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let ability_index = match self.ability {
            KrampusAbility::Kill => 0,
            KrampusAbility::DoNothing => 1
        };

        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            false,
            ControllerID::role(actor_ref, Role::Krampus, ability_index)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let ability_index = match self.ability {
            KrampusAbility::Kill => 0,
            KrampusAbility::DoNothing => 1
        };

        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Krampus, ability_index),
            true
        )
    }
    fn default_win_condition(self) -> WinCondition where super::RoleState: From<Self> {
        WinCondition::GameConclusionReached { win_if_any: vec![GameConclusion::NaughtyList].into_iter().collect() }
    }
}

impl GetClientRoleState<()> for Krampus {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) {}
}

impl Krampus {
    fn check_for_stalemate(game: &mut Game) {
        let remaining_roles = PlayerReference::all_players(game)
            .filter(|player| player.alive(game))
            .map(|player| player.role(game))
            .collect::<HashSet<Role>>();

        if remaining_roles.symmetric_difference(
            &vec![Role::SantaClaus, Role::Krampus].into_iter().collect()
        ).count() == 0 {
            for player in PlayerReference::all_players(game)
                .filter(|player| player.alive(game))
                .collect::<Vec<PlayerReference>>()
            {
                player.die(game, Grave::from_player_suicide(game, player));
            }
        }
    }
}