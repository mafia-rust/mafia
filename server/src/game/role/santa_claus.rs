use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::game_conclusion::GameConclusion;
use crate::game::phase::PhaseType;
use crate::game::win_condition::WinCondition;
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};

use super::{AbilitySelection, ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SantaClaus {
    pub next_ability: SantaAbility,
    pub ability_used_last_night: Option<SantaAbility>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SantaAbility{
    Naughty,
    #[default] Nice,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for SantaClaus {
    type ClientRoleState = SantaClaus;
    fn new_state(_: &Game) -> Self {
        Self::default()
    }
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Convert { return }

        match self.next_ability {
            SantaAbility::Nice => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game).into_iter();
                let targets = actor_visits.map(|v| v.target);

                for target_ref in targets {
                    match target_ref.win_condition(game).clone() {
                        WinCondition::GameConclusionReached { mut win_if_any } => {
                            win_if_any.insert(GameConclusion::NiceList);
                            target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });
    
                            target_ref.add_private_chat_message(game, ChatMessageVariant::AddedToNiceList);
                            actor_ref.set_role_state(game, Self {
                                ability_used_last_night: Some(SantaAbility::Nice),
                                ..self
                            });
                        }
                        WinCondition::RoleStateWon => {}
                    }
                }
            }
            SantaAbility::Naughty => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target_ref) = (
                    if let Some(visit) = actor_visits.first() {
                        Some(visit.target)
                    } else {
                        get_unlisted_living_players(game, actor_ref)
                            .into_iter()
                            .collect::<Vec<PlayerReference>>()
                            .choose(&mut thread_rng())
                            .cloned()
                    }
                ) else { return };

                match target_ref.win_condition(game).clone() {
                    WinCondition::GameConclusionReached { mut win_if_any } => {
                        win_if_any.insert(GameConclusion::NaughtyList);
                        target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });

                        target_ref.add_private_chat_message(game, ChatMessageVariant::AddedToNiceList);
                        actor_ref.set_role_state(game, Self {
                            ability_used_last_night: Some(SantaAbility::Naughty),
                            ..self
                        });
                    }
                    WinCondition::RoleStateWon => {}
                }
            }
        }
    }

    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        match self.next_ability {
            SantaAbility::Nice => {
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::role(actor_ref, Role::SantaClaus, 0),
                    super::AvailableAbilitySelection::new_player_list(
                        get_unlisted_living_players(game, actor_ref),
                        false,
                        Some(2)
                    ),
                    AbilitySelection::new_player_list(vec![]),
                    Detained::is_detained(game, actor_ref) || !actor_ref.alive(game),
                    Some(PhaseType::Obituary),
                    false,
                    vec_set!(actor_ref),
                )
            }
            SantaAbility::Naughty => {
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::role(actor_ref, Role::SantaClaus, 1),
                    super::AvailableAbilitySelection::new_player_list(
                        get_unlisted_living_players(game, actor_ref),
                        false,
                        Some(1)
                    ),
                    AbilitySelection::new_player_list(vec![]),
                    Detained::is_detained(game, actor_ref) || !actor_ref.alive(game),
                    Some(PhaseType::Obituary),
                    false,
                    vec_set!(actor_ref),
                )
            }
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let ability_id = match self.next_ability {
            SantaAbility::Nice => ControllerID::role(actor_ref, Role::SantaClaus, 0),
            SantaAbility::Naughty => ControllerID::role(actor_ref, Role::SantaClaus, 1)
        };

        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ability_id,
            false,
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            let mut new_state = self.clone();
            
            if let Some(ability) = self.ability_used_last_night {
                new_state.next_ability = match ability {
                    SantaAbility::Naughty => SantaAbility::Nice,
                    SantaAbility::Nice => SantaAbility::Naughty,
                };
                new_state.ability_used_last_night = None;
            }

            actor_ref.add_private_chat_message(game, ChatMessageVariant::NextSantaAbility { ability: new_state.next_ability });
            actor_ref.set_role_state(game, new_state);
        }
    }
}

fn get_unlisted_living_players(game: &Game, actor_ref: PlayerReference) -> VecSet<PlayerReference> {
    PlayerReference::all_players(game)
        .filter(|&p|
            actor_ref != p &&
            p.alive(game) &&
            !get_nice_listers(game).contains(&p) &&
            !get_naughty_listers(game).contains(&p)
        )
        .collect()
}

pub fn get_nice_listers(game: &Game) -> Vec<PlayerReference> {
    PlayerReference::all_players(game)
        .filter(|player|
            player.win_condition(game)
                .required_resolution_states_for_win()
                .is_some_and(|states| states.contains(&GameConclusion::NiceList))
        ).collect()
}

pub fn get_naughty_listers(game: &Game) -> Vec<PlayerReference> {
    PlayerReference::all_players(game)
        .filter(|player|
            player.win_condition(game)
                .required_resolution_states_for_win()
                .is_some_and(|states| states.contains(&GameConclusion::NaughtyList))
        ).collect()
}