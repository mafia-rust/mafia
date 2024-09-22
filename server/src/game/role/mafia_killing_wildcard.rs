use serde::{Serialize, Deserialize};

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{role_can_generate, Faction, RoleSet};
use crate::game::Game;

use super::{RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MafiaKillingWildcard{
    pub role: Role
}

pub type ClientRoleState = MafiaKillingWildcard;

impl Default for MafiaKillingWildcard {
    fn default() -> Self {
        Self {
            role: Role::MafiaKillingWildcard
        }
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for MafiaKillingWildcard {
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

impl MafiaKillingWildcard {
    fn become_role(&self, game: &mut Game, actor_ref: PlayerReference) {
        
        if self.role == Role::MafiaKillingWildcard {return;}

        if
            RoleSet::MafiaKilling.get_roles().contains(&self.role) &&
            role_can_generate(
                self.role, 
                &game.settings.enabled_roles, 
                &PlayerReference::all_players(game)
                    .map(|player_ref| player_ref.role(game))
                    .collect::<Vec<Role>>()
            )
        {
            actor_ref.set_role(game, self.role.default_state());
        }else{
            actor_ref.add_private_chat_message(game, ChatMessageVariant::WildcardConvertFailed{role: self.role.clone()})
        }
    }
}