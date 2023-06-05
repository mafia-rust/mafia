
use rand::seq::SliceRandom;

use crate::game::chat::ChatGroup;
use crate::game::phase::{PhaseType, PhaseState};
use crate::game::player::PlayerReference;
use crate::game::role::RoleData;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::verdict::Verdict;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::Priority;

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralEvil;
pub(super) const MAXIUMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::None;
pub(super) const TEAM: Option<Team> = None;


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    let &RoleData::Jester { lynched_yesterday } = actor_ref.role_data(game) else { unreachable!() };
    if priority != Priority::TopPriority {return;}
    if actor_ref.night_alive_tonight(game) {return;}

    if !lynched_yesterday {return}
    
    let all_killable_players: Vec<PlayerReference> = PlayerReference::all_players(game)
        .into_iter()
        .filter(|player_ref|{
            player_ref.night_alive_tonight(game) &&
            *player_ref != actor_ref &&
            player_ref.verdict(game) == Verdict::Guilty
        }).collect();

    let visit: Visit = match actor_ref.night_visits(game).first() {
        Some(v) => v.clone(),
        None => {
            //get random player from list
            let target_ref = all_killable_players.choose(&mut rand::thread_rng());

            let Some(target_ref) = target_ref else {return};
            Visit{
                target: *target_ref,
                astral: true,
                attack: true,
            }
        },
    };

    let target_ref = visit.target;
    target_ref.try_night_kill(game, crate::game::grave::GraveKiller::Role(super::Role::Jester), 3);

}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let &RoleData::Jester { lynched_yesterday } = actor_ref.role_data(game) else { unreachable!() };

    actor_ref != target_ref &&
    actor_ref.chosen_targets(game).is_empty() &&
    !actor_ref.alive(game) &&
    target_ref.alive(game) &&
    target_ref.verdict(game) != Verdict::Innocent &&
    lynched_yesterday
}
pub(super) fn do_day_action(_game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(_game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, true, true)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
    match game.current_phase() {
        &PhaseState::FinalWords { player_on_trial } => {
            if player_on_trial == actor_ref {
                actor_ref.set_role_data(game, RoleData::Jester { lynched_yesterday: true })
            }
        }
        PhaseState::Morning => {
            actor_ref.set_role_data(game, RoleData::Jester { lynched_yesterday: false })
        }
        _ => {}
    }
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}