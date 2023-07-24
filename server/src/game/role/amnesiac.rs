use std::vec;

use serde::{Serialize, Deserialize};

use crate::game::{chat::ChatGroup, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, RoleOutline};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amnesiac{
    pub role_outline: RoleOutline
}
impl Default for Amnesiac{
    fn default() -> Self {
        Self { role_outline: RoleOutline::Exact { role: Role::Amnesiac } }
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralChaos;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Amnesiac {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::None}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}
        let new_role_data = self.role_outline
            .get_random_role(&[RoleOutline::Exact { role: Role::Amnesiac }], &[])
            .unwrap_or(Role::Amnesiac)
            .default_state();
        if new_role_data.role() != Role::Amnesiac {
            actor_ref.set_role(game, new_role_data);
        }
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {

    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {

    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){

    }
}