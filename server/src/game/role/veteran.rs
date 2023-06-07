use crate::game::chat::night_message::NightInformation;
use crate::game::chat::ChatGroup;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleState, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = false;
pub(super) const WITCHABLE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if actor_ref.night_jailed(game) {return;}
    
    match priority {
        Priority::Unswappable => {
            let RoleState::Veteran { alerts_remaining, .. } = actor_ref.role_data(game) else {unreachable!()};
            
            if *alerts_remaining > 0 {
                if let Some(visit) = actor_ref.night_visits(game).first(){
                    if visit.target == actor_ref{
                        actor_ref.increase_defense_to(game, 1);
                        
                        let RoleState::Veteran { mut alerts_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!()};
                        alerts_remaining -= 1;
                        actor_ref.set_role_data(game, RoleState::Veteran { alerts_remaining, alerting_tonight: true });
                    }
                }
            }
        }
        Priority::Kill => {
            let RoleState::Veteran { alerting_tonight, .. } = actor_ref.role_data(game) else {unreachable!()};
            
            if !alerting_tonight {return}

            for other_player_ref in PlayerReference::all_players(game){
                for visit_index in 0..other_player_ref.night_visits(game).len(){
                    
                    let visit = &other_player_ref.night_visits(game)[visit_index];

                    if visit.target!=actor_ref || visit.astral {continue}
                    if other_player_ref.night_jailed(game){continue;}  //Attacking Jailed Player?

                    other_player_ref.push_night_messages(game,
                        NightInformation::VeteranAttackedYou 
                    );

                    //Kill
                    let killed = other_player_ref.try_night_kill(game, GraveKiller::Role(Role::Veteran), 2);
                    
                    actor_ref.push_night_messages(game, 
                        NightInformation::VeteranAttackedVisitor 
                    );

                    if !killed {
                        actor_ref.push_night_messages(game,
                            NightInformation::TargetSurvivedAttack 
                        );
                    }
                }
            }
        }
        _=>{}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleState::Veteran { alerts_remaining, .. } = actor_ref.role_data(game) else {unreachable!();};
    actor_ref == target_ref &&
    !actor_ref.night_jailed(game) &&
    *alerts_remaining > 0 &&
    actor_ref.chosen_targets(game).is_empty() &&
    actor_ref.alive(game)
}
pub(super) fn do_day_action(_game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {

}
pub(super) fn can_day_target(_game: &Game, _actor_ref: PlayerReference, _target: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, true, false)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
    let RoleState::Veteran { alerts_remaining, .. } = actor_ref.role_data(game).clone() else {unreachable!();};

    actor_ref.set_role_data(game, RoleState::Veteran { alerts_remaining, alerting_tonight: false });   
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}
