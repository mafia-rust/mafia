use serde::{Deserialize, Serialize};

use crate::game::{ability_input::{ability_selection::AbilitySelection, AbilityID, AbilityInput, ValidateAvailableSelection}, role_outline_reference::RoleOutlineReference, Game};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionSelection(
    pub Option<RoleOutlineReference>,
    pub Option<RoleOutlineReference>
);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvailableTwoRoleOutlineOptionSelection(pub Vec<Option<RoleOutlineReference>>);
impl ValidateAvailableSelection for AvailableTwoRoleOutlineOptionSelection{
    type Selection = TwoRoleOutlineOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &TwoRoleOutlineOptionSelection)->bool{
        self.0.contains(&selection.0) && 
        self.0.contains(&selection.1) && 
        (selection.0.is_none() || selection.0 != selection.1)
    }
}


impl AbilityInput{
    pub fn get_two_role_outline_option_selection_if_id(&self, id: AbilityID)->Option<TwoRoleOutlineOptionSelection>{
        if id != self.id() {return None};
        let AbilitySelection::TwoRoleOutlineOption { selection } = self.selection() else {return None};
        Some(selection)
    }
}