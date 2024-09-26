use serde::{Deserialize, Serialize};

use crate::game::{attack_power::DefensePower, role_list::Faction};


use super::RoleStateImpl;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Villager;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleActionChoice;
pub type ClientRoleState = Villager;

impl RoleStateImpl<ClientRoleState> for Villager {
    type RoleActionChoice = RoleActionChoice;
}

