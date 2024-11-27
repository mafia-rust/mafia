use serde::{Deserialize, Serialize};

use crate::game::{ability_input::ValidateAvailableSelection, role_outline_reference::RoleOutlineReference};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionSelection(
    pub Option<RoleOutlineReference>,
    pub Option<RoleOutlineReference>
);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableTwoRoleOutlineOptionSelection(pub Vec<Option<RoleOutlineReference>>);
impl ValidateAvailableSelection for AvailableTwoRoleOutlineOptionSelection{
    type Selection = TwoRoleOutlineOptionSelection;
    fn validate_selection(&self, selection: &TwoRoleOutlineOptionSelection)->bool{
        self.0.contains(&selection.0) && self.0.contains(&selection.1)
    }
}