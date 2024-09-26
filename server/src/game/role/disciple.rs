use serde::{Deserialize, Serialize};

use crate::game::{attack_power::DefensePower, role_list::Faction};
use super::RoleStateImpl;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Disciple;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleActionChoice;
pub type ClientRoleState = Disciple;

pub(super) const FACTION: Faction = Faction::Cult;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for Disciple {
    type RoleActionChoice = RoleActionChoice;
}