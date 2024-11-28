use serde::{Deserialize, Serialize};

use super::{selection_type::{
    one_player_option_selection::{AvailableOnePlayerOptionSelection, OnePlayerOptionSelection}, role_option_selection::{AvailableRoleOptionSelection, RoleOptionSelection}, two_player_option_selection::{AvailableTwoPlayerOptionSelection, TwoPlayerOptionSelection}, two_role_option_selection::{AvailableTwoRoleOptionSelection, TwoRoleOptionSelection}, two_role_outline_option_selection::{AvailableTwoRoleOutlineOptionSelection, TwoRoleOutlineOptionSelection}, BooleanSelection
}, ValidateAvailableSelection};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AbilitySelection{
    Unit,
    Boolean{selection: BooleanSelection},
    OnePlayerOption{selection: OnePlayerOptionSelection},
    TwoPlayerOption{selection: TwoPlayerOptionSelection},
    RoleOption{selection: RoleOptionSelection,},
    TwoRoleOption{selection: TwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: TwoRoleOutlineOptionSelection},
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AvailableAbilitySelection{
    Unit,
    Boolean,
    OnePlayerOption{selection: AvailableOnePlayerOptionSelection},
    TwoPlayerOption{selection: AvailableTwoPlayerOptionSelection},
    RoleOption{selection: AvailableRoleOptionSelection},
    TwoRoleOption{selection: AvailableTwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: AvailableTwoRoleOutlineOptionSelection},
}
impl ValidateAvailableSelection for AvailableAbilitySelection{
    type Selection = AbilitySelection;

    fn validate_selection(&self, selection: &Self::Selection)->bool {
        match self {
            Self::Unit => {true},
            Self::Boolean => {true},
            Self::OnePlayerOption{ selection: available } => {
                let AbilitySelection::OnePlayerOption{selection} = selection else {return false};
                return available.validate_selection(selection);
            },
            Self::TwoPlayerOption{ selection: available } => {
                let AbilitySelection::TwoPlayerOption{selection} = selection else {return false};
                return available.validate_selection(selection);
            },
            Self::RoleOption{ selection: available } => {
                let AbilitySelection::RoleOption{selection} = selection else {return false};
                return available.validate_selection(selection);
            },
            Self::TwoRoleOption{ selection: available } => {
                let AbilitySelection::TwoRoleOption{selection} = selection else {return false};
                return available.validate_selection(selection);
            },
            Self::TwoRoleOutlineOption{ selection: available } => {
                let AbilitySelection::TwoRoleOutlineOption{selection} = selection else {return false};
                return available.validate_selection(selection);
            },
        }
    }
}