
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::Priority;

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::MafiaDeception;
pub(super) const MAXIUMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Faction);


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if actor_ref.night_jailed(game) {return;}

    if priority != Priority::Deception {return;}

    let framer_visits = actor_ref.night_visits(game).clone();
    let Some(first_visit) = framer_visits.get(0) else {return};
    let Some(second_visit) = framer_visits.get(1) else {return};

    if first_visit.target.night_jailed(game) {
        actor_ref.push_night_messages(game, NightInformation::TargetJailed);
    }else{
        first_visit.target.set_night_appeared_suspicious(game, true);
        first_visit.target.set_night_appeared_visits(game, vec![second_visit.clone()]);
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    !actor_ref.night_jailed(game) &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    (
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref != target_ref &&
        !Team::same_team(
            actor_ref.role(game), 
            target_ref.role(game)
        )
    ) || 
    (
        actor_ref.chosen_targets(game).len() == 1
    )
    
}
pub(super) fn do_day_action(_game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(_game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    if target_refs.len() == 2 {
        vec![
            Visit{ target: target_refs[0], astral: false, attack: false }, 
            Visit{ target: target_refs[1], astral: false, attack: false }
        ]
    } else {
        Vec::new()
    }
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(_game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}