use crate::game::{chat::night_message::NightInformation, tag::Tag};
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleData, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::CovenUtility;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Faction);


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    //TODO NECRONOMICON

    if actor_ref.night_jailed(game) {return;}

    if priority != Priority::Deception {return}

    if let Some(visit) = actor_ref.night_visits(game).first(){
        let target_ref = visit.target;
        if target_ref.night_jailed(game){
            actor_ref.push_night_messages(game, NightInformation::TargetJailed);
            return
        }

        target_ref.set_night_silenced(game, true);
    }
}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
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
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Coven])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    if phase != PhaseType::Night {return;}

    let should_get_necronomicon = !PlayerReference::all_players(game).into_iter()
        .any(|p|p.role_data(game).has_necronomicon() && p.alive(game) && p.role(game) == Role::CovenLeader);
    
    if should_get_necronomicon {
        actor_ref.set_role_data(game, RoleData::VoodooMaster { necronomicon: true });
        for player_ref in PlayerReference::all_players(game) {
            if player_ref.role(game).faction_alignment().faction() == Faction::Coven{
                player_ref.push_player_tag(game, actor_ref, Tag::Necronomicon);
                player_ref.add_chat_message(game, ChatMessage::PlayerWithNecronomicon{ player_index: actor_ref.index() });
            }
        }
    }
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}
