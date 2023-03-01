use crate::game::chat::ChatGroup;
use crate::game::player::{Player, PlayerIndex};
use crate::game::role_list::FactionAlignment;
use crate::game::visit::Visit;
use crate::game::Game;

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::MafiaKilling;


pub(super) fn do_night_action(actor: PlayerIndex, priority: i8, game: &mut Game) {
    todo!();
}
pub(super) fn can_night_target(actor: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    todo!();
}
pub(super) fn do_day_action(actor: PlayerIndex, game: &mut Game) {
    todo!();
}
pub(super) fn can_day_target(actor: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    todo!();
}
pub(super) fn convert_targets_to_visits(actor: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game) -> Vec<Visit> {
    todo!();
}
pub(super) fn get_current_chat_groups(player: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
    todo!();
}