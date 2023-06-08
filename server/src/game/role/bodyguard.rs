
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::ChatGroup;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleState, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownProtective;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if actor_ref.night_jailed(game) {return;}

    match priority {
        Priority::Heal => {
            let Some(visit) = actor_ref.night_visits(game).first() else {return};
            let target_ref = visit.target;

            if target_ref.night_jailed(game){
                actor_ref.push_night_messages(game, NightInformation::TargetJailed);
                return
            }

            if actor_ref == target_ref {
                let RoleState::Bodyguard{mut self_shields_remaining, target_protected_ref, redirected_player_refs } = actor_ref.role_data(game).clone() else {unreachable!()};
                self_shields_remaining -= 1;
                target_ref.increase_defense_to(game, 2);
                actor_ref.set_role_data(game, RoleState::Bodyguard{self_shields_remaining, target_protected_ref, redirected_player_refs });
            }
        },
        Priority::Bodyguard => {

            let Some(visit) = actor_ref.night_visits(game).first() else {return};
            let target_ref = visit.target;

            if target_ref.night_jailed(game){
                actor_ref.push_night_messages(game, NightInformation::TargetJailed);
                return
            }

            if actor_ref != target_ref {
                
                let mut attackers: Vec<PlayerReference> = Vec::new();

                for attacker_ref in PlayerReference::all_players(game){
                    for attacker_visit in attacker_ref.night_appeared_visits(game){
                        if attacker_visit.attack && !attacker_visit.astral && attacker_visit.target == target_ref {
                            attackers.push(attacker_ref);
                        }
                    }
                }

                if attackers.is_empty() {return;}
                let RoleState::Bodyguard{self_shields_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
                let target_protected_ref = Some(target_ref);

                let mut redirected_player_refs = Vec::new();
                for attacker_ref in attackers {
                    if attacker_ref.night_jailed(game) {continue;}
                    let mut was_redirected = false;

                    let mut visits = Vec::new();
                    for attacker_visit in attacker_ref.night_visits(game){
                        if attacker_visit.target == target_ref && !attacker_visit.astral{
                            visits.push(Visit { target: actor_ref, astral: false, attack: attacker_visit.attack });
                            was_redirected = true;
                        }else{
                            visits.push(attacker_visit.clone());
                        }
                    }
                    attacker_ref.set_night_visits(game, visits);
                    if was_redirected {redirected_player_refs.push(attacker_ref);}
                }

                
                actor_ref.set_role_data(game, RoleState::Bodyguard{self_shields_remaining, target_protected_ref, redirected_player_refs});
            }
        },
        Priority::Kill => {
            let RoleState::Bodyguard{redirected_player_refs, ..} = actor_ref.role_data(game).clone() else {unreachable!()};
            
            for redirected_player_ref in redirected_player_refs {
                redirected_player_ref.try_night_kill(game, GraveKiller::Role(Role::Bodyguard), 2);
            }
        }
        Priority::Investigative => {
            let RoleState::Bodyguard{target_protected_ref, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
            
            if let Some(target_protected_ref) = target_protected_ref {
                actor_ref.push_night_messages(game, NightInformation::BodyguardProtected);
                target_protected_ref.push_night_messages(game, NightInformation::BodyguardProtectedYou);
            }
        }
        _ => {}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleState::Bodyguard { self_shields_remaining, .. } = actor_ref.role_data(game) else {unreachable!();};
    
    ((actor_ref == target_ref && *self_shields_remaining > 0) || actor_ref != target_ref) &&
    !actor_ref.night_jailed(game) &&
    actor_ref.chosen_targets(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game)
}
pub(super) fn do_day_action(_game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(_game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
    let RoleState::Bodyguard{ self_shields_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
    let redirected_player_refs = Vec::new();
    let target_protected_ref = None;
    actor_ref.set_role_data(game, RoleState::Bodyguard{ self_shields_remaining, redirected_player_refs, target_protected_ref });
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}