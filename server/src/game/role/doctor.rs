
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleState};

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

            let RoleState::Doctor{mut self_heals_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
            
            if actor_ref == target_ref {
                self_heals_remaining -= 1;
            }
            let target_healed_ref = Some(target_ref);
            target_ref.increase_defense_to(game, 2);

            actor_ref.set_role_data(game, RoleState::Doctor{self_heals_remaining, target_healed_ref});
        }
        Priority::Investigative => {
            let RoleState::Doctor{target_healed_ref, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
            
            if let Some(target_healed_ref) = target_healed_ref {
                if target_healed_ref.night_attacked(game){
                    
                    actor_ref.push_night_messages(game, NightInformation::DoctorHealed);
                    target_healed_ref.push_night_messages(game, NightInformation::DoctorHealedYou);
                }
            }
        }
        _ => {}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleState::Doctor { self_heals_remaining, .. } = actor_ref.role_data(game) else {unreachable!();};
    
    ((actor_ref == target_ref && *self_heals_remaining > 0) || actor_ref != target_ref) &&
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
    let RoleState::Doctor{self_heals_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
    let target_healed_index = None;
    actor_ref.set_role_data(game, RoleState::Doctor{self_heals_remaining, target_healed_ref: target_healed_index});
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}