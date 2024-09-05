use serde::Serialize;

use crate::game::role_list::Faction;
use super::RoleStateImpl;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Disciple;

pub(super) const FACTION: Faction = Faction::Cult;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Disciple {}