use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::ValidateAvailableSelection, role::Role}, vec_set::VecSet};


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOptionSelection(pub Option<Role>);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableRoleOptionSelection(pub VecSet<Option<Role>>);
impl ValidateAvailableSelection for AvailableRoleOptionSelection{
    type Selection = RoleOptionSelection;
    fn validate_selection(&self, selection: &RoleOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}