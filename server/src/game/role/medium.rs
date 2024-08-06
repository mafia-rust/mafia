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

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium{
    pub seances_remaining: u8,
    pub seanced_target: Option<PlayerReference>
}
impl Default for Medium{
    fn default() -> Self {
        Self { seances_remaining: 2, seanced_target: None}
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Medium {
    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
    }
    fn can_select(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.seanced_target {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: None, seances_remaining: self.seances_remaining}));
            } else {
                actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: Some(target_ref), seances_remaining: self.seances_remaining }));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: Some(target_ref), seances_remaining: self.seances_remaining }));
        }
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.current_phase().is_day() &&
        self.seances_remaining > 0 && 
        actor_ref != target_ref &&
        !actor_ref.alive(game) && target_ref.alive(game) && 
        game.current_phase().phase() != PhaseType::Night
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![PlayerGroup::Dead])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);

        if game.current_phase().is_night() && actor_ref.alive(game) {
            out.push(PlayerGroup::Dead);
        }
        out
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Obituary => {
                self.seanced_target = None;
                actor_ref.set_role_state(game, RoleState::Medium(self));
            },
            PhaseType::Night => {
                if let Some(seanced) = self.seanced_target {
                    if seanced.alive(game) && !actor_ref.alive(game){
                
                        PlayerGroup::Dead.send_chat_message(game,
                            ChatMessageVariant::MediumHauntStarted{ medium: actor_ref.index(), player: seanced.index() }
                        );

                        self.seances_remaining -= 1;
                    }
                }
                actor_ref.set_role_state(game, RoleState::Medium(self));
            },
            _=>{}
        }
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
