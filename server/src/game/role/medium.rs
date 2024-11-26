use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::detained::Detained;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::{RoleStateImpl, RoleState};

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


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Medium {
    type ClientRoleState = Medium;
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
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Dead]);

        if 
            (game.current_phase().phase() == PhaseType::Obituary) &&
            actor_ref.alive(game)
        {
            out.insert(ChatGroup::Dead);
        }
        out
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);

        if 
            (
                (
                    !Detained::is_detained(game, actor_ref) &&
                    game.current_phase().phase() == PhaseType::Night
                ) || 
                game.current_phase().phase() == PhaseType::Obituary
            ) &&
            actor_ref.alive(game)
        {
            out.insert(ChatGroup::Dead);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Discussion => {
                self.seanced_target = None;
                actor_ref.set_role_state(game, self);
            },
            PhaseType::Night => {
                if let Some(seanced) = self.seanced_target {
                    if seanced.alive(game) && !actor_ref.alive(game){
                
                        game.add_message_to_chat_group(ChatGroup::Dead,
                            ChatMessageVariant::MediumHauntStarted{ medium: actor_ref.index(), player: seanced.index() }
                        );

                        self.seances_remaining = self.seances_remaining.saturating_sub(1);
                    }
                }
                actor_ref.set_role_state(game, self);
            },
            _=>{}
        }
    }
    fn on_any_death(self, game: &mut Game, _actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        dead_player_ref.add_private_chat_message(game, ChatMessageVariant::MediumExists);
    }
}
