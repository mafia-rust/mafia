use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{
    game::{
        ability_input::{
            ability_selection::AbilitySelection, AbilityID,
            AbilityInput, ValidateAvailableSelection
        },
        role::Role, Game
    },
    vec_set::VecSet
};

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

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTwoRoleOptionSelection{
    pub available_roles: VecSet<Option<Role>>,
    
    pub can_choose_duplicates: bool
}
impl ValidateAvailableSelection for AvailableTwoRoleOptionSelection{
    type Selection = TwoRoleOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &TwoRoleOptionSelection)->bool{
        if !self.can_choose_duplicates && selection.same_role(){
            return false
        }
        self.available_roles.contains(&selection.0) && self.available_roles.contains(&selection.1)
    }
}
impl PartialOrd for AvailableTwoRoleOptionSelection{
    fn partial_cmp(&self, _other: &Self)->Option<std::cmp::Ordering>{
        Some(Ordering::Equal)
    }
}
impl Ord for AvailableTwoRoleOptionSelection{
    fn cmp(&self, _other: &Self)->Ordering{
        Ordering::Equal
    }
}




impl AbilityInput{
    pub fn get_two_role_option_selection_if_id(&self, id: AbilityID)->Option<TwoRoleOptionSelection>{
        if id != self.id() {return None};
        let AbilitySelection::TwoRoleOption { selection } = self.selection() else {return None};
        Some(selection)
    }
}