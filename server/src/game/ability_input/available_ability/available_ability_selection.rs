use serde::{Deserialize, Serialize};

use crate::{
    game::{
        ability_input::*,
        player::PlayerReference,
        role::Role,
        role_outline_reference::RoleOutlineReference,
        Game
    },
    vec_set::VecSet
};

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
impl AvailableAbilitySelection{
    pub fn new_unit()->Self{
        Self::Unit
    }
    pub fn new_boolean()->Self{
        Self::Boolean
    }
    pub fn new_one_player_option(players: VecSet<Option<PlayerReference>>)->Self{
        Self::OnePlayerOption{selection: AvailableOnePlayerOptionSelection(players)}
    }
    pub fn new_two_player_option(
        first: VecSet<Option<PlayerReference>>,
        second: VecSet<Option<PlayerReference>>,
        can_choose_duplicates: bool
    )->Self{
        Self::TwoPlayerOption{selection: AvailableTwoPlayerOptionSelection{
            available_first_player: first,
            available_second_player: second,
            can_choose_duplicates 
        }}
    }
    pub fn new_role_option(selection: VecSet<Option<Role>>)->Self{
        Self::RoleOption{selection: AvailableRoleOptionSelection(selection)}
    }
    pub fn new_two_role_option(first: VecSet<Option<Role>>, can_choose_duplicates: bool)->Self{
        Self::TwoRoleOption{selection: AvailableTwoRoleOptionSelection{
            available_roles: first,
            can_choose_duplicates
        }}
    }
    pub fn new_two_role_outline_option(available_outlines: VecSet<Option<RoleOutlineReference>>)->Self{
        Self::TwoRoleOutlineOption{selection: AvailableTwoRoleOutlineOptionSelection(available_outlines)}
    }
    pub fn new_kira(selection: AvailableKiraSelection)->Self{
        Self::Kira{selection}
    }
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