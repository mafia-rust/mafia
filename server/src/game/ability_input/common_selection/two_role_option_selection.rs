use serde::{Deserialize, Serialize};

use crate::game::role::Role;

use super::AvailableSelection;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOptionSelection(pub Option<Role>, pub Option<Role>);
impl TwoRoleOptionSelection{
    pub fn any_in_common(&self, other: &TwoRoleOptionSelection) -> bool{
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
#[serde(rename_all = "camelCase")]
pub struct AvailableTwoRoleOptionSelection{
    pub available_roles: Vec<Option<Role>>,
    
    pub can_choose_duplicates: bool
}
impl AvailableSelection for AvailableTwoRoleOptionSelection{
    type Selection = TwoRoleOptionSelection;
    fn validate_selection(&self, selection: &TwoRoleOptionSelection)->bool{
        if !self.can_choose_duplicates && selection.same_role(){
            return false
        }
        self.available_roles.contains(&selection.0) && self.available_roles.contains(&selection.1)
    }
}