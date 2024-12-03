use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::{ability_selection::AbilitySelection, ControllerID, AbilityInput, ValidateAvailableSelection}, role::Role, Game}, vec_set::VecSet};


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOptionSelection(pub Option<Role>);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableRoleOptionSelection(pub VecSet<Option<Role>>);
impl ValidateAvailableSelection for AvailableRoleOptionSelection{
    type Selection = RoleOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &RoleOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}


impl AbilityInput{
    pub fn get_role_option_selection_if_id(&self, id: ControllerID)->Option<RoleOptionSelection>{
        if id != self.id() {return None};
        let AbilitySelection::RoleOption { selection } = self.selection() else {return None};
        Some(selection)
    }
}