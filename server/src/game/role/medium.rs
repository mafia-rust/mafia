use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::Game;

use super::common_role::RoleActionChoiceOnePlayer;
use super::RoleStateImpl;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium{
    pub seances_remaining: u8,
    pub seance_selection: <Self as RoleStateImpl>::RoleActionChoice
}

impl Default for Medium{
    fn default() -> Self {
        Self { seances_remaining: 2, seance_selection: RoleActionChoiceOnePlayer::default() }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Medium {
    type ClientRoleState = Medium;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        let Some(target_ref) = action_choice.player else {
            if !game.current_phase().is_day() {return;}
            self.seance_selection = action_choice;
            actor_ref.set_role_state(game, self);
            return;
        };
        if !(
            game.current_phase().is_day() &&
            self.seances_remaining > 0 && 
            actor_ref != target_ref &&
            !actor_ref.alive(game) && target_ref.alive(game) && 
            game.current_phase().phase() != PhaseType::Night
        ) {
            return;
        }
        self.seance_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Dead])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);

        if game.current_phase().is_night() && actor_ref.alive(game) {
            out.insert(ChatGroup::Dead);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Obituary => {
                self.seance_selection = RoleActionChoiceOnePlayer::default();
                actor_ref.set_role_state(game, self);
            },
            PhaseType::Night => {
                if let Some(seanced) = self.seance_selection.player {
                    if seanced.alive(game) && !actor_ref.alive(game){
                
                        game.add_message_to_chat_group(ChatGroup::Dead,
                            ChatMessageVariant::MediumHauntStarted{ medium: actor_ref.index(), player: seanced.index() }
                        );

                        self.seances_remaining -= 1;
                    }
                }
                actor_ref.set_role_state(game, self);
            },
            _=>{}
        }
    }
}
