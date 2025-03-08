use serde::Serialize;

use crate::game::attack_power::DefensePower;


use super::RoleStateImpl;

/*
    All the code is in components/pathologist_info_dump.rs because while its just an 
    on_any_death function, it needs to run before any other on_any_death functions.
*/
#[derive(Debug, Clone, Serialize, Default)]
pub struct Pathologist;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Pathologist {
    type ClientRoleState = Pathologist;
}