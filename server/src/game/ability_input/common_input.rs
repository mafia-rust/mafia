use serde::{Deserialize, Serialize};

use crate::game::{player::PlayerReference, role::Role, role_outline_reference::RoleOutlineReference};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitInput;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanInput(pub bool);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePlayerOptionInput(pub Option<PlayerReference>);
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOptionSelection(pub Option<PlayerReference>);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOptionInput(pub Option<Role>, pub Option<Role>);
impl TwoRoleOptionInput{
    pub fn any_in_common(&self, other: &TwoRoleOptionInput) -> bool{
        (self.0.is_some() && self.0 == other.0) || 
        (self.0.is_some() && self.0 == other.1) || 
        (self.1.is_some() && self.1 == other.0) || 
        (self.1.is_some() && self.1 == other.1)
    }
    pub fn same_role(&self) -> bool{
        self.0.is_some() && self.0 == self.1 
    }
    pub fn contains(&self, role: Role) -> bool{
        self.0 == Some(role) || self.1 == Some(role)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionInput(pub Option<RoleOutlineReference>, pub Option<RoleOutlineReference>);
