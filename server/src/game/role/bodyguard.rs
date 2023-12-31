
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleState, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bodyguard {
    self_shields_remaining: u8,
    target_protected_ref: Option<PlayerReference>,
    redirected_player_refs: Vec<PlayerReference>
}

impl Default for Bodyguard {
    fn default() -> Self {
        Self { 
            self_shields_remaining: 1, 
            target_protected_ref: None, 
            redirected_player_refs: Vec::new()
        }
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownProtective;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Bodyguard {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
    
        match priority {
            Priority::Bodyguard => {
    
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;
                if actor_ref == target_ref {return}
    
                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }

                let mut redirected_player_refs = vec![];
                let mut target_protected_ref = None;
                for attacker_ref in PlayerReference::all_players(game){
                    let mut new_visits = vec![];
                    for mut attacking_visit in attacker_ref.night_visits(game).clone(){
                        if attacking_visit.target == target_ref && !attacking_visit.astral && attacking_visit.attack {
                            attacking_visit.target = actor_ref;
                            redirected_player_refs.push(attacker_ref);
                            target_protected_ref = Some(target_ref);
                        }
                        new_visits.push(attacking_visit);
                    }
                    attacker_ref.set_night_visits(game, new_visits);
                }

                actor_ref.set_role_state(game, RoleState::Bodyguard(Bodyguard {
                    self_shields_remaining: self.self_shields_remaining, 
                    target_protected_ref, 
                    redirected_player_refs
                }));
                
            },
            Priority::Heal => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;
    
                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }
    
                if actor_ref == target_ref {
                    let self_shields_remaining = self.self_shields_remaining - 1;
                    target_ref.increase_defense_to(game, 2);
                    actor_ref.set_role_state(game, RoleState::Bodyguard(Bodyguard{ self_shields_remaining, target_protected_ref: self.target_protected_ref, redirected_player_refs: self.redirected_player_refs }));
                }
            },
            Priority::Kill => {
                for redirected_player_ref in self.redirected_player_refs {
                    redirected_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Bodyguard), 2, false);
                }
            }
            Priority::Investigative => {
                if let Some(target_protected_ref) = self.target_protected_ref {
                    actor_ref.push_night_message(game, ChatMessage::TargetWasAttacked);
                    target_protected_ref.push_night_message(game, ChatMessage::YouWereProtected);
                }
            }
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        ((actor_ref == target_ref && self.self_shields_remaining > 0) || actor_ref != target_ref) &&
        !actor_ref.night_jailed(game) &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        let redirected_player_refs = Vec::new();
        let target_protected_ref = None;
        actor_ref.set_role_state(game, RoleState::Bodyguard(Bodyguard { self_shields_remaining: self.self_shields_remaining, redirected_player_refs, target_protected_ref }));
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
