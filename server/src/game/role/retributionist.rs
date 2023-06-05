use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleData};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownSupport;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if actor_ref.night_jailed(game) {return;}

    if priority != Priority::Necromancy {return;}
    
    let retributionist_visits = actor_ref.night_visits(game).clone();
    let Some(first_visit) = retributionist_visits.get(0) else {return};
    let Some(second_visit) = retributionist_visits.get(1) else {return};

    if first_visit.target.alive(game) {return;}
    
    first_visit.target.set_night_visits(
        game,
        first_visit.target.role(game).convert_targets_to_visits(game,
            first_visit.target, vec![second_visit.target]
        )
    );

    let RoleData::Retributionist { mut used_bodies } = actor_ref.role_data(game).clone() else {unreachable!()};
    used_bodies.push(first_visit.target);
    actor_ref.set_role_data(game, RoleData::Retributionist { used_bodies });
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleData::Retributionist { used_bodies } = actor_ref.role_data(game) else {unreachable!()};

    !actor_ref.night_jailed(game) &&
    actor_ref.alive(game) &&
    ((
        actor_ref.chosen_targets(game).is_empty() &&
        !target_ref.alive(game) &&
        target_ref.role(game).faction_alignment().faction() == Faction::Town &&
        !used_bodies.iter().any(|p| *p == target_ref)
    ) || (
        actor_ref != target_ref &&
        actor_ref.chosen_targets(game).len() == 1 &&
        target_ref.alive(game)

    ))
}
pub(super) fn do_day_action(_game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(_game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    if target_refs.len() == 2 {
        vec![
            Visit{target: target_refs[0], astral: false, attack: false}, 
            Visit{target: target_refs[1], astral: true, attack: false}
        ]
    } else {
        Vec::new()
    }
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(_game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}
