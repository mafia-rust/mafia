use std::vec;

use serde::{Serialize, Deserialize};

use crate::game::{chat::ChatGroup, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::RoleOutline;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl, Role};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amnesiac{
    pub role_outline: RoleOutline
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Amnesiac {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}
        if !actor_ref.alive(game) {return;}
        let new_role_data = self.role_outline
            .get_random_role(&game.settings.excluded_roles, &[])
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
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        false
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}