
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerReference};
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleData};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownProtective;
pub(super) const MAXIUMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if *actor_ref.night_roleblocked(game) {return}

    match priority {
        6 => {
            let Some(visit) = actor_ref.night_visits(game).first() else {return};
            let target_ref: PlayerReference = visit.target.clone();
            let RoleData::Doctor{mut self_heals_remaining, mut target_healed_ref } = actor_ref.role_data(game).clone() else {unreachable!()};
            
            if actor_ref == target_ref {
                self_heals_remaining -= 1;
            }
            target_healed_ref = Some(target_ref);
            target_ref.increase_defense_to(game, 2);

            actor_ref.set_role_data(game, RoleData::Doctor{self_heals_remaining, target_healed_ref});
        }
        10 => {
            let RoleData::Doctor{self_heals_remaining, target_healed_ref } = actor_ref.role_data(game).clone() else {unreachable!()};
            
            if let Some(target_healed_ref) = target_healed_ref {
                if *target_healed_ref.night_attacked(game){
                    
                    actor_ref.push_night_messages(game, ChatMessage::NightInformation { night_information: NightInformation::DoctorHealed });
                    target_healed_ref.push_night_messages(game, ChatMessage::NightInformation { night_information: NightInformation::DoctorHealedYou });
                }
            }
        }
        _ => {}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    actor_ref != target_ref &&
    actor_ref.chosen_targets(game).len() < 1 &&
    *actor_ref.alive(game) &&
    *target_ref.alive(game)
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target: PlayerReference) -> bool {
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
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    let RoleData::Doctor{self_heals_remaining, target_healed_ref: mut target_healed_index } = actor_ref.role_data(game).clone() else {unreachable!()};
    target_healed_index = None;
    actor_ref.set_role_data(game, RoleData::Doctor{self_heals_remaining, target_healed_ref: target_healed_index});
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}