use serde::Serialize;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::game_conclusion::GameConclusion;
use crate::game::phase::PhaseType;
use crate::game::win_condition::WinCondition;
use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};
use super::{AbilitySelection, ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SantaClaus {
    pub ability_used_last_night: Option<SantaListKind>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SantaListKind{
    Naughty,
    #[default] Nice,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for SantaClaus {
    type ClientRoleState = SantaClaus;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Convert { return }

        match self.get_next_santa_ability() {
            SantaListKind::Nice => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game).into_iter();
                let targets = actor_visits.map(|v| v.target);

                for target_ref in targets {
                    let WinCondition::GameConclusionReached { mut win_if_any } = target_ref.win_condition(game).clone() else {
                        actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                        continue
                    };

                    if
                        !AttackPower::ArmorPiercing.can_pierce(target_ref.defense(game)) ||
                        !get_eligible_players(game, actor_ref).contains(&target_ref)
                    {
                        actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                        continue;
                    }
                    
                    win_if_any.insert(GameConclusion::NiceList);
                    target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });
                    target_ref.push_night_message(game, ChatMessageVariant::AddedToNiceList);

                    actor_ref.set_role_state(game, Self {
                        ability_used_last_night: Some(SantaListKind::Nice),
                        ..self
                    });
                }
            }
            SantaListKind::Naughty => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game).into_iter();
                let targets = actor_visits.map(|v| v.target);

                for target_ref in targets {
                    let WinCondition::GameConclusionReached { mut win_if_any } = target_ref.win_condition(game).clone() else {
                        actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                        continue
                    };

                    if
                        !AttackPower::ArmorPiercing.can_pierce(target_ref.defense(game)) ||
                        !get_eligible_players(game, actor_ref).contains(&target_ref)
                    {
                        actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                        continue;
                    }
                    
                    win_if_any.insert(GameConclusion::NaughtyList);
                    target_ref.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any });
                    target_ref.push_night_message(game, ChatMessageVariant::AddedToNaughtyList);

                    actor_ref.set_role_state(game, Self {
                        ability_used_last_night: Some(SantaListKind::Naughty),
                        ..self
                    });
                    
                    for krampus in PlayerReference::all_players(game) {
                        if krampus.role(game) != Role::Krampus { continue }

                        krampus.push_night_message(game, 
                            ChatMessageVariant::SantaAddedPlayerToNaughtyList { player: target_ref }
                        );
                    }
                }
            }
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        match self.get_next_santa_ability() {
            SantaListKind::Nice => {
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::role(actor_ref, Role::SantaClaus, 0),
                    super::AvailableAbilitySelection::new_player_list(
                        get_selectable_players(game, actor_ref),
                        false,
                        Some(1)
                    ),
                    AbilitySelection::new_player_list(vec![]),
                    Detained::is_detained(game, actor_ref) ||
                    actor_ref.ability_deactivated_from_death(game),
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
                        Some(1)
                    ),
                    AbilitySelection::new_player_list(vec![]),
                    Detained::is_detained(game, actor_ref) ||
                    actor_ref.ability_deactivated_from_death(game),
                    Some(PhaseType::Obituary),
                    false,
                    vec_set!(actor_ref),
                )
            }
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {        
        match self.get_next_santa_ability() {
            SantaListKind::Nice => {
                crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::SantaClaus, 0),
                    true,
                )
            }
            SantaListKind::Naughty => {
                crate::game::role::common_role::convert_controller_selection_to_visits(
                    game,
                    actor_ref,
                    ControllerID::role(actor_ref, Role::SantaClaus, 1),
                    true,
                )
            }
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                actor_ref.add_private_chat_message(game,
                    ChatMessageVariant::NextSantaAbility { ability: self.get_next_santa_ability() }
                );
            },
            _ => {}
        }
    }
    fn default_win_condition(self) -> WinCondition where super::RoleState: From<Self> {
        WinCondition::GameConclusionReached { win_if_any: vec![GameConclusion::NiceList].into_iter().collect() }
    }
}

impl SantaClaus {
    fn get_next_santa_ability(&self)->SantaListKind{
        match self.ability_used_last_night {
            Some(SantaListKind::Nice) => SantaListKind::Naughty,
            _ => SantaListKind::Nice,
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