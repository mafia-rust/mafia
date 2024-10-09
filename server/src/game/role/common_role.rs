use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::{
    chat::ChatGroup, components::puppeteer_marionette::PuppeteerMarionette, modifiers::{ModifierType, Modifiers}, phase::{PhaseState, PhaseType}, player::PlayerReference, resolution_state::ResolutionState, role_list::Faction, visit::Visit, win_condition::WinCondition, Game
};

use super::{journalist::Journalist, medium::Medium, same_evil_team, Role, RoleState};


pub(super) fn default_action_choice_one_player_is_valid(game: &Game, actor_ref: PlayerReference, target: Option<PlayerReference>, can_choose_self: bool) -> bool {
    let Some(target) = target else {return true};

    (can_choose_self || actor_ref != target) &&
    target.alive(game) &&
    !default_action_choice_boolean_is_valid(game, actor_ref) &&
    !same_evil_team(game, actor_ref, target)
}
pub(super) fn default_action_choice_boolean_is_valid(game: &Game, actor_ref: PlayerReference) -> bool {
    !actor_ref.night_jailed(game) &&
    actor_ref.alive(game)
}
pub(super) fn default_action_choice_two_players_is_valid(
    game: &Game,
    actor_ref: PlayerReference, 
    two_players: Option<(PlayerReference, PlayerReference)>, 
    can_choose_self: (bool, bool),
    can_double_choose: bool
) -> bool {
    let Some((target_a, target_b)) = two_players else {return true};

    !default_action_choice_boolean_is_valid(game, actor_ref) &&
    target_a.alive(game) && 
    target_b.alive(game) &&
    (can_double_choose || target_a != target_b) &&
    (can_choose_self.0 || (actor_ref != target_a && !same_evil_team(game, actor_ref, target_a))) &&
    (can_choose_self.1 || (actor_ref != target_b && !same_evil_team(game, actor_ref, target_b)))
    
}

pub(super) fn convert_action_choice_to_visits(target: Option<PlayerReference>, attack: bool) -> Vec<Visit> {
    if let Some(target) = target {
        vec![Visit{ target: target, attack }]
    } else {
        Vec::new()
    }
}
pub(super) fn convert_action_choice_to_visits_two_players(two_players: Option<(PlayerReference, PlayerReference)>, attack: bool) -> Vec<Visit> {
    if let Some((a,b)) = two_players {
        vec![
            Visit{ target: a, attack },
            Visit{ target: b, attack }
        ]
    } else {
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, mut night_chat_groups: Vec<ChatGroup>) -> HashSet<ChatGroup> {
    if 
        !actor_ref.alive(game) && 
        !Modifiers::modifier_is_enabled(game, ModifierType::DeadCanChat)
    {
        if PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&actor_ref){
            return vec![ChatGroup::Dead, ChatGroup::Puppeteer].into_iter().collect();
        }
        return vec![ChatGroup::Dead].into_iter().collect();
    }
    if actor_ref.night_silenced(game){
        return HashSet::new();
    }

    match game.current_phase() {
        PhaseState::Briefing => HashSet::new(),
        PhaseState::Obituary => {
            let mut evil_chat_groups = HashSet::new();

            if PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&actor_ref) {
                evil_chat_groups.insert(ChatGroup::Puppeteer);
            }

            match actor_ref.role(game).faction() {
                Faction::Mafia => {
                    evil_chat_groups.insert(ChatGroup::Mafia);
                },
                Faction::Cult => {
                    evil_chat_groups.insert(ChatGroup::Cult);
                },
                _ => {}
            }

            evil_chat_groups
        },
        PhaseState::Discussion 
        | PhaseState::Nomination {..}
        | PhaseState::Judgement {..} 
        | PhaseState::FinalWords {..}
        | PhaseState::Dusk => vec![ChatGroup::All].into_iter().collect(),
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All].into_iter().collect()
            } else {
                HashSet::new()
            }
        },
        PhaseState::Night => {
            let mut out = vec![];
            if PlayerReference::all_players(game)
                .any(|med|{
                    match med.role_state(game) {
                        RoleState::Medium(Medium{ seance_selection: RoleActionChoiceOnePlayer{player: Some(seanced_target)}, .. }) => {
                            actor_ref == *seanced_target
                        },
                        _ => false
                    }
                })
            {
                out.push(ChatGroup::Dead);
            }
            if PlayerReference::all_players(game)
                .any(|p|
                    match p.role_state(game) {
                        RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                            *interviewed_target_ref == actor_ref
                        },
                        _ => false
                    }
                )
            {
                out.push(ChatGroup::Interview);
            }


            let mut jail_or_night_chats = if actor_ref.night_jailed(game){
                vec![ChatGroup::Jail]
            }else{
                
                if PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&actor_ref){
                    night_chat_groups.push(ChatGroup::Puppeteer);
                }

                match actor_ref.role(game).faction() {
                    Faction::Mafia => {
                        night_chat_groups.push(ChatGroup::Mafia);
                        night_chat_groups
                    },
                    Faction::Cult => {
                        night_chat_groups.push(ChatGroup::Cult);
                        night_chat_groups
                    },
                    _ => {
                        night_chat_groups
                    }
                }
            };


            out.append(&mut jail_or_night_chats);
            out.into_iter().collect()
        },
    }
}
pub(super) fn get_current_receive_chat_groups(game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !actor_ref.alive(game){
        out.push(ChatGroup::Dead);
    }

    if actor_ref.role(game).faction() == Faction::Mafia {
        out.push(ChatGroup::Mafia);
    }
    if actor_ref.role(game).faction() == Faction::Cult {
        out.push(ChatGroup::Cult);
    }
    if PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&actor_ref){
        out.push(ChatGroup::Puppeteer);
    }
    if actor_ref.night_jailed(game){
        out.push(ChatGroup::Jail);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|med|{
                match med.role_state(game) {
                    RoleState::Medium(Medium{ seance_selection: RoleActionChoiceOnePlayer{player: Some(seanced_target)}, .. }) => {
                        actor_ref == *seanced_target
                    },
                    _ => false
                }
            })
    {
        out.push(ChatGroup::Dead);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|p|
                match p.role_state(game) {
                    RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                        *interviewed_target_ref == actor_ref
                    },
                    _ => false
                }
            )
    {
        out.push(ChatGroup::Interview);
    }

    out.into_iter().collect()
}

///Only works for roles that win based on end game condition
pub(super) fn default_win_condition(role: Role) -> WinCondition {
    WinCondition::ResolutionStateReached{win_if_any: 
        match role.faction(){
            Faction::Mafia => vec![ResolutionState::Mafia],
            Faction::Cult => vec![ResolutionState::Cult],
            Faction::Town => vec![ResolutionState::Town],
            Faction::Fiends => vec![ResolutionState::Fiends],
            Faction::Neutral => match role {
                Role::Minion | Role::Scarecrow => {
                    ResolutionState::all().into_iter().filter(|end_game_condition|
                        match end_game_condition {
                            ResolutionState::Town | ResolutionState::Draw => false,
                            _ => true
                        }
                    ).collect()
                },
                Role::Politician => vec![ResolutionState::Politician],
                _ => {return WinCondition::RoleStateWon;}
            },
        }.into_iter().collect()
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoiceOnePlayer{
    pub player: Option<PlayerReference>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoiceTwoPlayers{
    pub two_players: Option<(PlayerReference, PlayerReference)>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoiceRole{
    pub role: Option<Role>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoiceBool{
    pub boolean: bool,
}



pub mod macros {
    #[macro_export]
    macro_rules! on_phase_start_reset_night_selection {
        (
            $self:ident,
            $game:ident,
            $actor_ref:ident,
            $phase:ident
        ) => {
            match $phase {
                crate::game::phase::PhaseType::Obituary => {
                    $self.night_selection = <Self as RoleStateImpl>::RoleActionChoice::default();
                    $actor_ref.set_role_state($game, $self);
                }
                _ => {}
            }
        }
    }
}