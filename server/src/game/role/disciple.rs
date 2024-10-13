use serde::Serialize;

use crate::game::{attack_power::DefensePower, role_list::Faction};
use super::RoleStateImpl;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Disciple;

pub type ClientRoleState = Disciple;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Disciple {
    type ClientRoleState = Disciple;
}