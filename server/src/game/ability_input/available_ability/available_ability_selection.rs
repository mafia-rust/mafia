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
    ThreePlayerOption{selection: AvailableThreePlayerOptionSelection},
    PlayerList{selection: AvailablePlayerListSelection},
    RoleOption{selection: AvailableRoleOptionSelection},
    TwoRoleOption{selection: AvailableTwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: AvailableTwoRoleOutlineOptionSelection},
    String,
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
        first: VecSet<PlayerReference>,
        second: VecSet<PlayerReference>,
        can_choose_duplicates: bool,
        can_choose_none: bool
    )->Self{
        Self::TwoPlayerOption{selection: AvailableTwoPlayerOptionSelection{
            available_first_players: first,
            available_second_players: second,
            can_choose_duplicates,
            can_choose_none
        }}
    }
    pub fn new_three_player_option(
        first: VecSet<Option<PlayerReference>>,
        second: VecSet<Option<PlayerReference>>,
        third: VecSet<Option<PlayerReference>>,
        can_choose_duplicates: bool
    )->Self{
        Self::ThreePlayerOption{selection: AvailableThreePlayerOptionSelection{
            available_first_players: first,
            available_second_players: second,
            available_third_players: third,
            can_choose_duplicates
        }}
    }
    pub fn new_player_list(players: VecSet<PlayerReference>, can_choose_duplicates: bool, max: Option<u8>)->Self{
        Self::PlayerList{selection: AvailablePlayerListSelection{
            available_players: players,
            can_choose_duplicates,
            max_players: max
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
    pub fn new_string()->Self{
        Self::String
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
            Self::ThreePlayerOption { selection: available } => {
                let AbilitySelection::ThreePlayerOption{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            },
            Self::PlayerList { selection: available } => {
                let AbilitySelection::PlayerList{selection} = selection else {return false};
                return available.validate_selection(game, selection);
            }
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
            Self::String => {true},
            Self::Kira{ selection: available} => {
                let AbilitySelection::Kira { selection } = selection else {return false};
                return available.validate_selection(game, selection);
            }
        }
    }
}