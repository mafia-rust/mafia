
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, RoleState, Role};

pub(super) const SUSPICIOUS: bool = false;
pub(super) const WITCHABLE: bool = true;
pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownPower;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mayor {
    pub revealed: bool,
}
impl Default for Mayor {
    fn default() -> Self {
        Self { revealed: false}
    }
}
impl RoleStateImpl for Mayor {
    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {

    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, _target_ref: PlayerReference) {

        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::MayorRevealed { player_index: actor_ref.index() });

        actor_ref.set_role_state(game, RoleState::Mayor(Mayor{revealed: true}));
        for player in PlayerReference::all_players(game){
            player.insert_role_label(game, actor_ref, Role::Mayor);
        }
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        self.revealed == false &&
        actor_ref == target_ref &&
        actor_ref.alive(game) &&
        PhaseType::Night != game.current_phase().phase()
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {

    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}