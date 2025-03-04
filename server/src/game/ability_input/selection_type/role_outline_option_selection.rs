use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::{ability_selection::AbilitySelection, ControllerID, AbilityInput, ValidateAvailableSelection}, role_outline_reference::RoleOutlineReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOutlineOptionSelection(
    pub Option<RoleOutlineReference>,
);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableRoleOutlineOptionSelection(pub VecSet<Option<RoleOutlineReference>>);
impl ValidateAvailableSelection for AvailableRoleOutlineOptionSelection{
    type Selection = RoleOutlineOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &RoleOutlineOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}

impl PartialOrd for AvailableRoleOutlineOptionSelection{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        Some(self.cmp(other))
    }
}
impl Ord for AvailableRoleOutlineOptionSelection{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        self.0.cmp(&other.0)
    }
}


impl AbilityInput{
    pub fn get_role_outline_option_selection_if_id(&self, id: ControllerID)->Option<RoleOutlineOptionSelection>{
        if id != self.id() {return None};
        let AbilitySelection::RoleOutlineOption { selection } = self.selection() else {return None};
        Some(selection)
    }
}