
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerIndex, PlayerReference};
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
    if game.get_unchecked_player(actor_index).night_variables.roleblocked {return}

    match priority {
        6 => {
            let Some(visit) = game.get_unchecked_player(actor_index).night_variables.visits.first() else {return};
            let target_index: PlayerIndex = visit.target.clone();
            let RoleData::Doctor{mut self_heals_remaining, mut target_healed_index } = game.get_unchecked_player(actor_index).role_data().clone() else {unreachable!()};
            
            if actor_index == target_index {
                self_heals_remaining -= 1;
            }
            target_healed_index = Some(target_index);
            game.get_unchecked_mut_player(target_index).night_variables.increase_defense_to(2);

            game.get_unchecked_mut_player(actor_index).set_role_data(RoleData::Doctor{self_heals_remaining, target_healed_index});
        }
        10 => {
            let RoleData::Doctor{self_heals_remaining, target_healed_index } = game.get_unchecked_player(actor_index).role_data().clone() else {unreachable!()};
            
            if let Some(target_healed_index) = target_healed_index {
                if game.get_unchecked_player(target_healed_index).night_variables.attacked{
                    
                    game.get_unchecked_mut_player(actor_index).add_chat_message(ChatMessage::NightInformation { night_information: NightInformation::DoctorHealed });
                    game.get_unchecked_mut_player(target_healed_index).add_chat_message(ChatMessage::NightInformation { night_information: NightInformation::DoctorHealedYou });
                }
            }
        }
        _ => {}
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    actor_index != target_index &&
    game.get_unchecked_player(actor_index).chosen_targets().len() < 1 &&
    *game.get_unchecked_player(actor_index).alive() &&
    *game.get_unchecked_player(target_index).alive()
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target: PlayerIndex) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(actor_index, targets, game, false, false)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    let actor = game.get_unchecked_mut_player(actor_index);
    let RoleData::Doctor{self_heals_remaining, mut target_healed_index } = actor.role_data().clone() else {unreachable!()};
    target_healed_index = None;
    game.get_unchecked_mut_player(actor_index).set_role_data(RoleData::Doctor{self_heals_remaining, target_healed_index});
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}