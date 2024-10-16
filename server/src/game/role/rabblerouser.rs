use serde::Serialize;

use crate::game::{
    attack_power::DefensePower,
    components::pitchfork::Pitchfork,
    event::before_role_switch::BeforeRoleSwitch,
    player::PlayerReference, 
    Game
};


use super::RoleStateImpl;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Rabblerouser;

impl RoleStateImpl for Rabblerouser {
    type ClientRoleState = Rabblerouser;
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Pitchfork::add_pitchfork(game, actor_ref);
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, event: BeforeRoleSwitch) {
        if event.player() == actor_ref {
            Pitchfork::remove_pitchfork(game, actor_ref);
        }
    }
}

