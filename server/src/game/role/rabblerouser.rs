use serde::Serialize;

use crate::game::{
    attack_power::DefensePower, components::pitchfork::Pitchfork, player::PlayerReference, Game
};


use super::{RoleState, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Rabblerouser;

impl RoleStateImpl for Rabblerouser {
    type ClientRoleState = Rabblerouser;
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Pitchfork::add_pitchfork(game, actor_ref);
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _old: RoleState, _new: RoleState) {
        if player == actor_ref {
            Pitchfork::remove_pitchfork(game, actor_ref);
        }
    }
    /// handled in pitchfork
    fn attack_data(&self, _game: &Game, _actor_ref: PlayerReference) -> crate::game::attack_type::AttackData {
        crate::game::attack_type::AttackData::none()
    }
}

