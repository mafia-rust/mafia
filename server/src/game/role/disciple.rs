use serde::Serialize;
use crate::vec_set;
use crate::{game::{attack_power::DefensePower, components::insider_group::InsiderGroupID}, vec_set::VecSet};
use super::RoleStateImpl;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Disciple;

pub type ClientRoleState = Disciple;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Disciple {
    type ClientRoleState = Disciple;
    fn default_revealed_groups(self) -> VecSet<InsiderGroupID> {
        vec_set![InsiderGroupID::Cult]
    }
}