use serde::{Deserialize, Serialize};

use crate::game::{
    player::PlayerReference, role::Role,
    role_outline_reference::RoleOutlineReference
};

use super::{selection_type::{
    kira_selection::KiraSelection, one_player_option_selection::OnePlayerOptionSelection, role_option_selection::RoleOptionSelection, two_player_option_selection::TwoPlayerOptionSelection, two_role_option_selection::TwoRoleOptionSelection, two_role_outline_option_selection::TwoRoleOutlineOptionSelection, BooleanSelection
}, StringSelection, ThreePlayerOptionSelection};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AbilitySelection{
    Unit,
    Boolean{selection: BooleanSelection},
    OnePlayerOption{selection: OnePlayerOptionSelection},
    TwoPlayerOption{selection: TwoPlayerOptionSelection},
    ThreePlayerOption{selection: ThreePlayerOptionSelection},
    RoleOption{selection: RoleOptionSelection,},
    TwoRoleOption{selection: TwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: TwoRoleOutlineOptionSelection},
    String{selection: StringSelection},
    Kira{selection: KiraSelection}
}
impl AbilitySelection{
    pub fn new_unit()->Self{
        Self::Unit
    }
    pub fn new_boolean(selection: bool)->Self{
        Self::Boolean{selection: BooleanSelection(selection)}
    }
    pub fn new_one_player_option(selection: Option<PlayerReference>)->Self{
        Self::OnePlayerOption{selection: OnePlayerOptionSelection(selection)}
    }
    pub fn new_two_player_option(first: Option<PlayerReference>, second: Option<PlayerReference>)->Self{
        Self::TwoPlayerOption{selection: TwoPlayerOptionSelection(first, second)}
    }
    pub fn new_three_player_option(first: Option<PlayerReference>, second: Option<PlayerReference>, third: Option<PlayerReference>)->Self{
        Self::ThreePlayerOption{selection: ThreePlayerOptionSelection(first, second, third)}
    }
    pub fn new_role_option(selection: Option<Role>)->Self{
        Self::RoleOption{selection: RoleOptionSelection(selection)}
    }
    pub fn new_two_role_option(first: Option<Role>, second: Option<Role>)->Self{
        Self::TwoRoleOption{selection: TwoRoleOptionSelection(first, second)}
    }
    pub fn new_two_role_outline_option(first: Option<RoleOutlineReference>, second: Option<RoleOutlineReference>)->Self{
        Self::TwoRoleOutlineOption{selection: TwoRoleOutlineOptionSelection(first, second)}
    }
    pub fn new_string(string: String)->Self{
        Self::String{selection: StringSelection(string)}
    }
    pub fn new_kira(selection: KiraSelection)->Self{
        Self::Kira{selection}
    }
}

