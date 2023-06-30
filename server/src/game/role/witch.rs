use serde::Serialize;

use crate::game::chat::{ChatGroup};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl};

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::CovenPower;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

#[derive(Clone, Debug, Default, Serialize)]
pub struct Witch;

impl RoleStateImpl for Witch {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Coven}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {Some(Team::Coven)}


    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {

    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Coven])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}
