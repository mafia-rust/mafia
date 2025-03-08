use serde::{Deserialize, Serialize};

use crate::game::{
    player::PlayerReference, role::Role,
    role_outline_reference::RoleOutlineReference
};

use super::{selection_type::{
    kira_selection::KiraSelection, role_option_selection::RoleOptionSelection, two_player_option_selection::TwoPlayerOptionSelection, two_role_option_selection::TwoRoleOptionSelection, two_role_outline_option_selection::TwoRoleOutlineOptionSelection, BooleanSelection
}, IntegerSelection, PlayerListSelection, StringSelection};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AbilitySelection{
    Unit,
    Boolean{selection: BooleanSelection},
    TwoPlayerOption{selection: TwoPlayerOptionSelection},
    PlayerList{selection: PlayerListSelection},
    RoleOption{selection: RoleOptionSelection,},
    TwoRoleOption{selection: TwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: TwoRoleOutlineOptionSelection},
    String{selection: StringSelection},
    Integer{selection: IntegerSelection},
    Kira{selection: KiraSelection}
}
impl AbilitySelection{
    pub fn new_unit()->Self{
        Self::Unit
    }
    pub fn new_boolean(selection: bool)->Self{
        Self::Boolean{selection: BooleanSelection(selection)}
    }
    pub fn new_two_player_option(selection: Option<(PlayerReference, PlayerReference)>)->Self{
        Self::TwoPlayerOption{selection: TwoPlayerOptionSelection(selection)}
    }
    pub fn new_player_list(selection: Vec<PlayerReference>)->Self{
        Self::PlayerList{selection: PlayerListSelection(selection)}
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
    pub fn new_integer(integer: i8)->Self{
        Self::Integer{selection: IntegerSelection(integer)}
    }
    pub fn new_kira(selection: KiraSelection)->Self{
        Self::Kira{selection}
    }
}

