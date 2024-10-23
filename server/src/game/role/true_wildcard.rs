use serde::{Serialize, Deserialize};

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::role_can_generate;
use crate::game::Game;

use super::{RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrueWildcard{
    pub role: Role
}

impl Default for TrueWildcard {
    fn default() -> Self {
        Self {
            role: Role::TrueWildcard
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for TrueWildcard {
    type ClientRoleState = TrueWildcard;
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                if !actor_ref.alive(game) {return;}
                self.become_role(game, actor_ref);
            },
            _ => {}
        }
    }
}

impl TrueWildcard {
    fn become_role(&self, game: &mut Game, actor_ref: PlayerReference) {

        if self.role == Role::TrueWildcard {return;}

        if 
            role_can_generate(
                self.role, 
                &game.settings.enabled_roles, 
                &[]
            )
        {
            actor_ref.set_role_and_win_condition_and_revealed_group(game, self.role.default_state());
        }else{
            actor_ref.add_private_chat_message(game, ChatMessageVariant::WildcardConvertFailed{role: self.role.clone()})
        }
    }
}