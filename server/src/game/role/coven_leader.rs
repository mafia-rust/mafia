use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerIndex};
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;

use super::Priority;

pub(super) const DEFENSE: u8 = 1;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::CovenPower;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Faction);


pub(super) fn do_night_action(actor_index: PlayerIndex, priority: Priority, game: &mut Game) {
    //TODO
}
pub(super) fn can_night_target(actor_index: PlayerIndex, target_index: PlayerIndex, game: &Game) -> bool {
    crate::game::role::common_role::can_night_target(actor_index, target_index, game)
}
pub(super) fn do_day_action(actor_index: PlayerIndex, game: &mut Game) {

}
pub(super) fn can_day_target(actor_index: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(actor_index: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(actor_index, targets, game, false, false)
}
pub(super) fn get_current_send_chat_groups(actor_index: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(actor_index, game, vec![ChatGroup::Coven])
}
pub(super) fn on_phase_start(actor_index: PlayerIndex, phase: PhaseType, game: &mut Game){
    //TODO give necronomicon
}