use std::vec;

use serde::{Serialize, Deserialize};

use crate::game::chat::{ChatMessageVariant, RecipientLike};
use crate::game::grave::GraveReference;
use crate::game::{player_group::PlayerGroup, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::{role_can_generate, Faction};
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wildcard{
    pub role: Role
}
impl Default for Wildcard {
    fn default() -> Self {
        Self {
            role: Role::Wildcard
        }
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Wildcard {
    
    


    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_select(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        false
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                if !actor_ref.alive(game) {return;}
                self.become_role(game, actor_ref);
            },
            _ => {}
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}

impl Wildcard {
    fn become_role(&self, game: &mut Game, actor_ref: PlayerReference) {

        if self.role == Role::Wildcard {return;}

        if 
            role_can_generate(
                self.role, 
                &game.settings.excluded_roles, 
                &PlayerReference::all_players(game)
                    .map(|player_ref| player_ref.role(game))
                    .collect::<Vec<Role>>()
            )
        {
            actor_ref.set_role(game, self.role.default_state());
        }else{
            actor_ref.add_chat_message(game, ChatMessageVariant::WildcardConvertFailed{role: self.role.clone()})
        }
    }
}