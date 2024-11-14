use serde::{Deserialize, Serialize};

use crate::game::role_outline_reference::RoleOutlineReference;

use super::AvailableSelection;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionSelection(
    pub Option<RoleOutlineReference>,
    pub Option<RoleOutlineReference>
);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableTwoRoleOutlineOptionSelection(pub Vec<Option<RoleOutlineReference>>);
impl AvailableSelection for AvailableTwoRoleOutlineOptionSelection{
    type Selection = TwoRoleOutlineOptionSelection;
    fn validate_selection(&self, selection: &TwoRoleOutlineOptionSelection)->bool{
        self.0.contains(&selection.0) && self.0.contains(&selection.1)
    }
}