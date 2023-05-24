
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::{Grave, GraveRole};
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
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::MafiaDeception;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Faction);


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if *actor_ref.night_jailed(game) {return}
    if *actor_ref.night_roleblocked(game) {return}

    let RoleData::Janitor { cleans_remaining, cleaned_ref } = actor_ref.role_data(game) else {unreachable!()};
    let cleans_remaining = cleans_remaining.to_owned();
    if(cleans_remaining <= 0){return}

    match priority {
        Priority::Deception=>{
            let Some(visit) = actor_ref.night_visits(game).first() else{return};

            let target_ref = visit.target;
            if *target_ref.night_jailed(game) {
                actor_ref.push_night_messages(game, NightInformation::TargetJailed);
            }else{
                target_ref.set_night_grave_role(game, GraveRole::Cleaned);
                target_ref.set_night_grave_will(game, "".to_owned());
                actor_ref.set_role_data(game, RoleData::Janitor { cleans_remaining: cleans_remaining-1, cleaned_ref: Some(target_ref) });
            }
        },
        Priority::Investigative=>{
            let RoleData::Janitor { cleans_remaining, cleaned_ref } = actor_ref.role_data(game) else {unreachable!()};
            if let Some(cleaned_ref) = cleaned_ref {
                if *cleaned_ref.night_died(game) {
                    actor_ref.push_night_messages(game, NightInformation::PlayerRoleAndWill{
                        role: cleaned_ref.role(game),
                        will: cleaned_ref.will(game).to_string(),
                    });
                }
            }
        },
        _ => {}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleData::Janitor { cleans_remaining, cleaned_ref } = actor_ref.role_data(game) else {unreachable!()};
    crate::game::role::common_role::can_night_target(game, actor_ref, target_ref) && *cleans_remaining > 0
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
            let RoleData::Janitor { cleans_remaining, cleaned_ref } = actor_ref.role_data(game) else {unreachable!()};
    actor_ref.set_role_data(game, RoleData::Janitor { cleans_remaining: *cleans_remaining, cleaned_ref: None });
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}