use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::detained::Detained;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;
use crate::vec_set;

use super::{
    AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap, PlayerListSelection, Role, RoleStateImpl
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium{
    pub seances_remaining: u8,
    pub seanced_target: Option<PlayerReference>
}

impl Default for Medium{
    fn default() -> Self {
        Self { seances_remaining: 3, seanced_target: None}
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Medium {
    type ClientRoleState = Medium;
    fn new_state(game: &Game) -> Self {
        Self{
            seances_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Medium, 0),
            AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|p| p.alive(game))
                    .filter(|p| *p != actor_ref)
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.alive(game) || self.seances_remaining <= 0,
            Some(PhaseType::Night),
            false,
            vec_set!(actor_ref)
        )
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
                let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Medium, 0)
                ) else {return};

                let Some(target) = target.first() else {return};
                
                self.seances_remaining = self.seances_remaining.saturating_sub(1);
                self.seanced_target = Some(target.clone());
                
                game.add_message_to_chat_group(ChatGroup::Dead,
                    ChatMessageVariant::MediumHauntStarted{
                        medium: actor_ref.index(),
                        player: target.index()
                    }
                );
                
                actor_ref.set_role_state(game, self);
            },
            _=>{}
        }
    }
    fn on_any_death(self, game: &mut Game, _actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        dead_player_ref.add_private_chat_message(game, ChatMessageVariant::MediumExists);
    }
}
