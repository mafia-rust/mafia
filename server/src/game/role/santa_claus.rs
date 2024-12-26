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

use crate::game::visit::{Visit, VisitTag};
use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};

use super::{AbilitySelection, ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SantaClaus {
    pub next_ability: SantaListKind,
    pub ability_used_last_night: Option<SantaListKind>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SantaListKind{
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
            SantaListKind::Nice => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game).into_iter();
                let targets = actor_visits.map(|v| v.target);

                for target_ref in targets {
                    if !get_eligible_players(game, actor_ref).contains(&target_ref) { continue }

                    match target_ref.win_condition(game).clone() {
                        WinCondition::GameConclusionReached { mut win_if_any } => {
                            win_if_any.insert(GameConclusion::NiceList);
                            target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });

                            target_ref.add_private_chat_message(game, ChatMessageVariant::AddedToNiceList);
                            actor_ref.set_role_state(game, Self {
                                ability_used_last_night: Some(SantaListKind::Nice),
                                ..self
                            });
                        }
                        WinCondition::RoleStateWon => {}
                    }
                }
            }
            SantaListKind::Naughty => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game).into_iter();
                let targets = actor_visits.map(|v| v.target);

                for target_ref in targets {
                    match target_ref.win_condition(game).clone() {
                        WinCondition::GameConclusionReached { mut win_if_any } => {
                            win_if_any.insert(GameConclusion::NaughtyList);
                            target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });
    
    
                            let krampus_list: Vec<PlayerReference> = PlayerReference::all_players(game)
                                .filter(|player| player.role(game) == Role::Krampus)
                                .collect();
    
                            if !krampus_list.is_empty() {
                                target_ref.add_private_chat_message(game, ChatMessageVariant::AddedToNaughtyList);
                            }
                            for krampus in krampus_list {
                                krampus.add_private_chat_message(game, 
                                    ChatMessageVariant::SantaAddedPlayerToNaughtyList { player: target_ref }
                                );
                            }
                            actor_ref.set_role_state(game, Self {
                                ability_used_last_night: Some(SantaListKind::Naughty),
                                ..self
                            });
                        }
                        WinCondition::RoleStateWon => {}
                    }
                }
            }
        }
    }

    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        match self.next_ability {
            SantaListKind::Nice => {
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::role(actor_ref, Role::SantaClaus, 0),
                    super::AvailableAbilitySelection::new_player_list(
                        get_selectable_players(game, actor_ref),
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
            SantaListKind::Naughty => {
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::role(actor_ref, Role::SantaClaus, 1),
                    super::AvailableAbilitySelection::new_player_list(
                        get_selectable_players(game, actor_ref),
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
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        match self.next_ability {
            SantaListKind::Nice => {
                crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::SantaClaus, 0),
                    false,
                )
            }
            SantaListKind::Naughty => {
                let visits = crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::SantaClaus, 1),
                    false,
                );
                let eligible_targets: Vec<PlayerReference> = visits.iter()
                    .map(|v| v.target)
                    .filter(|t| get_eligible_players(game, actor_ref).contains(t))
                    .collect();

                let mut eligible_options = get_eligible_players(game, actor_ref)
                    .into_iter()
                    .filter(|e| !eligible_targets.contains(e))
                    .collect::<Vec<PlayerReference>>();

                eligible_options.shuffle(&mut thread_rng());

                let mut targets: Vec<PlayerReference> = eligible_targets.into_iter().chain(eligible_options).collect();

                targets.truncate(2);

                targets.into_iter()
                    .map(|target| Visit::new(actor_ref, target, false, VisitTag::Role))
                    .collect()
            }
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Obituary || phase == PhaseType::Night {
            let mut new_state = self.clone();
            
            if let Some(ability) = self.ability_used_last_night {
                new_state.next_ability = match ability {
                    SantaListKind::Naughty => SantaListKind::Nice,
                    SantaListKind::Nice => SantaListKind::Naughty,
                };
                new_state.ability_used_last_night = None;
            }

            actor_ref.add_private_chat_message(game, ChatMessageVariant::NextSantaAbility { ability: new_state.next_ability });
            actor_ref.set_role_state(game, new_state);
        }
    }
}

fn get_selectable_players(game: &Game, actor_ref: PlayerReference) -> VecSet<PlayerReference> {
    PlayerReference::all_players(game)
        .filter(|&p|
            actor_ref != p &&
            p.alive(game)
        )
        .collect()
}

fn get_eligible_players(game: &Game, actor_ref: PlayerReference) -> VecSet<PlayerReference> {
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