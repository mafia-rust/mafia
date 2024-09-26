use serde::{Deserialize, Serialize};

use crate::game::{
    attack_power::DefensePower,
    components::pitchfork::Pitchfork,
    event::before_role_switch::BeforeRoleSwitch,
    player::PlayerReference, role_list::Faction, Game};


use super::RoleStateImpl;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct RabbleRouser;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleActionChoice{
    target: Option<PlayerReference>
}
pub type ClientRoleState = RabbleRouser;

impl RoleStateImpl<ClientRoleState> for RabbleRouser {
    type RoleActionChoice = RoleActionChoice;
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Pitchfork::add_pitchfork(game, actor_ref);
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, event: BeforeRoleSwitch) {
        if event.player() == actor_ref {
            Pitchfork::remove_pitchfork(game, actor_ref);
        }
    }
}

