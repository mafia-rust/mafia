use serde::{Serialize, Deserialize};

use crate::game::attack_power::DefensePower;
use crate::game::role_list::Faction;

use super::RoleStateImpl;


#[derive(Debug, Clone, Serialize, Default)]
pub struct MadeMan;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleActionChoice;
pub type ClientRoleState = MadeMan;

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for MadeMan {
    type RoleActionChoice = RoleActionChoice;
}