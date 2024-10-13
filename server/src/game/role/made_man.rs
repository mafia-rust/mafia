use serde::Serialize;

use crate::game::attack_power::DefensePower;


use super::RoleStateImpl;


#[derive(Debug, Clone, Serialize, Default)]
pub struct MadeMan;

pub type ClientRoleState = MadeMan;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for MadeMan {
    type ClientRoleState = MadeMan;
}