use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerIndex, self, PlayerReference};
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleData, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = false;
pub(super) const WITCHABLE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    match priority {
        1 => {
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = actor_ref.deref(game).role_data() else {unreachable!()};
            
            if *alerts_remaining > 0 {
                if let Some(visit) = actor_ref.deref(game).night_variables.visits.get(0){
                    if visit.target == actor_ref{
                        actor_ref.deref_mut(game).night_variables.increase_defense_to(1);
                        
                        let RoleData::Veteran { mut alerts_remaining, mut alerting_tonight } = actor_ref.deref(game).role_data().clone() else {unreachable!()};
                        alerts_remaining -= 1;
                        alerting_tonight = true;
                        actor_ref.deref_mut(game).set_role_data(RoleData::Veteran{alerts_remaining, alerting_tonight });
                    }
                }
            }
        }
        9 => {
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = actor_ref.deref(game).role_data() else {unreachable!()};
            
            if !alerting_tonight {return}

            for other_player_ref in PlayerReference::all_players(game){
                for visit_index in 0..other_player_ref.deref(game).night_variables.visits.len(){
                    
                    let visit = other_player_ref.deref(game).night_variables.visits.get(visit_index).unwrap();

                    if visit.target!=actor_ref || visit.astral {continue}

                    other_player_ref.deref_mut(game).add_chat_message(ChatMessage::NightInformation{ 
                        night_information: NightInformation::VeteranAttackedYou 
                    });

                    //Kill
                    let killed = Player::try_night_kill(game, other_player_ref, GraveKiller::Role(Role::Veteran), 2);
                    
                    let actor = other_player_ref.deref_mut(game);
                    actor.add_chat_message(ChatMessage::NightInformation{ 
                        night_information: NightInformation::VeteranAttackedVisitor 
                    });

                    if !killed {
                        actor.add_chat_message(ChatMessage::NightInformation{ 
                            night_information: NightInformation::TargetSurvivedAttack 
                        });
                    }
                }
            }
        }
        _=>{}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleData::Veteran { alerts_remaining, alerting_tonight } = actor_ref.deref(game).role_data() else {unreachable!();};
    actor_ref == target_ref &&
    *alerts_remaining > 0 &&
    actor_ref.deref(game).chosen_targets().len() < 1 &&
    *actor_ref.deref(game).alive()
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference) {

}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target: PlayerIndex) -> bool {
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
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    let actor = actor_ref.deref_mut(game);
    
    let RoleData::Veteran { alerts_remaining, mut alerting_tonight } = actor.role_data().clone() else {unreachable!();};
    
    alerting_tonight = false;

    actor_ref.deref_mut(game).set_role_data(RoleData::Veteran{alerts_remaining, alerting_tonight });   
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}
