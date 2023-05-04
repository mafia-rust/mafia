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
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_player(actor_index).role_data() else {unreachable!()};
            
            if *alerts_remaining > 0 {
                if let Some(visit) = game.get_unchecked_player(actor_index).night_variables.visits.get(0){
                    if visit.target == actor_index{
                        game.get_unchecked_mut_player(actor_index).night_variables.increase_defense_to(1);
                        
                        let RoleData::Veteran { mut alerts_remaining, mut alerting_tonight } = game.get_unchecked_player(actor_index).role_data().clone() else {unreachable!()};
                        alerts_remaining -= 1;
                        alerting_tonight = true;
                        game.get_unchecked_mut_player(actor_index).set_role_data(RoleData::Veteran{alerts_remaining, alerting_tonight });
                    }
                }
            }
        }
        9 => {
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_player(actor_index).role_data() else {unreachable!()};
            
            if !alerting_tonight {return}

            for player_index in 0..game.players.len(){
                let player_index = (player_index as PlayerIndex).clone();
                for visit_index in 0..game.get_unchecked_player(player_index).night_variables.visits.len(){
                    let visit = game.get_unchecked_player(player_index).night_variables.visits.get(visit_index).unwrap();

                    if !visit.target==actor_index || visit.astral {continue}

                    game.get_unchecked_mut_player(player_index).add_chat_message(ChatMessage::NightInformation{ 
                        night_information: NightInformation::VeteranAttackedYou 
                    });

                    //Kill
                    let killed = Player::try_night_kill(game, player_index, GraveKiller::Role(Role::Veteran), 2);
                    
                    let actor = game.get_unchecked_mut_player(actor_index);
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
    let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_player(actor_index).role_data() else {unreachable!();};
    actor_index == target_index &&
    *alerts_remaining > 0 &&
    game.get_unchecked_player(actor_index).chosen_targets().len() < 1 &&
    *game.get_unchecked_player(actor_index).alive()
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference) {

}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target: PlayerIndex) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, game, true, false)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    let actor = game.get_unchecked_mut_player(actor_index);
    
    let RoleData::Veteran { alerts_remaining, mut alerting_tonight } = actor.role_data().clone() else {unreachable!();};
    
    alerting_tonight = false;

    game.get_unchecked_mut_player(actor_index).set_role_data(RoleData::Veteran{alerts_remaining, alerting_tonight });   
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}
