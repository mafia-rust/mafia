
use serde::Serialize;

use crate::game::chat::{ChatMessageVariant, RecipientLike};
use crate::game::player_group::PlayerGroup;
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl, RoleState};

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mayor {
    pub revealed: bool
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Mayor {
    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, _target_ref: PlayerReference) {

        if !actor_ref.alive(game) || !game.current_phase().is_day() {
            return;
        }

        PlayerGroup::All.send_chat_message(game, ChatMessageVariant::MayorRevealed { player_index: actor_ref.index() });
        PlayerGroup::All.insert_role_label(game, actor_ref);

        actor_ref.set_role_state(game, RoleState::Mayor(Mayor{
            revealed: true
        }));
        game.count_votes_and_start_trial();
    }
    fn can_select(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool{
        false
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool{
        game.current_phase().is_day() &&
        !self.revealed &&
        actor_ref == target_ref &&
        actor_ref.alive(game) &&
        PhaseType::Night != game.current_phase().phase()
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit>{
        vec![]
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup>{
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup>{
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool{
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}