use serde::{Deserialize, Serialize};

use crate::game::Game;

use super::{selection_type::{
    kira_selection::{AvailableKiraSelection, KiraSelection}, one_player_option_selection::{AvailableOnePlayerOptionSelection, OnePlayerOptionSelection}, role_option_selection::{AvailableRoleOptionSelection, RoleOptionSelection}, two_player_option_selection::{AvailableTwoPlayerOptionSelection, TwoPlayerOptionSelection}, two_role_option_selection::{AvailableTwoRoleOptionSelection, TwoRoleOptionSelection}, two_role_outline_option_selection::{AvailableTwoRoleOutlineOptionSelection, TwoRoleOutlineOptionSelection}, BooleanSelection
}, ValidateAvailableSelection};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
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
    Kira{selection: KiraSelection}
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
    Kira{selection: AvailableKiraSelection}
}
impl ValidateAvailableSelection for AvailableAbilitySelection{
    type Selection = AbilitySelection;

    fn validate_selection(&self, game: &Game, selection: &Self::Selection)->bool {
        match self {
            Self::Unit => {true},
            Self::Boolean => {true},
            Self::OnePlayerOption{ selection: available } => {
                let AbilitySelection::OnePlayerOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::TwoPlayerOption{ selection: available } => {
                let AbilitySelection::TwoPlayerOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::RoleOption{ selection: available } => {
                let AbilitySelection::RoleOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::TwoRoleOption{ selection: available } => {
                let AbilitySelection::TwoRoleOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::TwoRoleOutlineOption{ selection: available } => {
                let AbilitySelection::TwoRoleOutlineOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::Kira{ selection: available} => {
                let AbilitySelection::Kira { selection } = selection else {return false};
                return available.validate_selection(game, selection);
            }
        }
    }
}