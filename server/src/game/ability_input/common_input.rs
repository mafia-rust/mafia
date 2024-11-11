use serde::{Deserialize, Serialize};

use crate::game::{player::PlayerReference, role_outline_reference::RoleOutlineReference};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitInput;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanInput(pub bool);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePlayerOptionInput(pub Option<PlayerReference>);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionInput(pub Option<RoleOutlineReference>, pub Option<RoleOutlineReference>);
