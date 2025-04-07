use serde::{Deserialize, Serialize};

use super::{selection_type::{
    kira_selection::KiraSelection, role_option_selection::RoleOptionSelection, two_player_option_selection::TwoPlayerOptionSelection, two_role_option_selection::TwoRoleOptionSelection, two_role_outline_option_selection::TwoRoleOutlineOptionSelection, BooleanSelection
}, *, ChatMessageSelection, IntegerSelection, PlayerListSelection, StringSelection, UnitSelection};

macro_rules! selection_kinds {
    (
        $($name:ident: $available_kind:ident, $kind:ident);*
    ) => {
        #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag="type", content="selection")]
        pub enum AvailableAbilitySelection {
            $($name($available_kind)),*
        }

        $(
            impl From<$available_kind> for AvailableAbilitySelection {
                fn from(value: $available_kind) -> AvailableAbilitySelection {
                    AvailableAbilitySelection::$name(value)
                }
            }
        )*

        #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag="type", content="selection")]
        pub enum AbilitySelection{
            $($name($kind)),*
        }

        $(
            impl From<$kind> for AbilitySelection {
                fn from(value: $kind) -> AbilitySelection {
                    AbilitySelection::$name(value)
                }
            }
        )*
        

        impl AvailableAbilitySelection{
            pub fn validate_selection(&self, game: &Game, selection: &AbilitySelection)->bool {
                match self {
                    $(Self::$name(available) => {
                        let AbilitySelection::$name(selection) = selection else {return false};
                        available.validate_selection(game, selection)
                    }),*
                }
            }
        }
    }
}

selection_kinds! {
    Unit: AvailableUnitSelection, UnitSelection;
    Boolean: AvailableBooleanSelection, BooleanSelection;
    TwoPlayerOption: AvailableTwoPlayerOptionSelection, TwoPlayerOptionSelection;
    PlayerList: AvailablePlayerListSelection, PlayerListSelection;
    RoleOption: AvailableRoleOptionSelection, RoleOptionSelection;
    TwoRoleOption: AvailableTwoRoleOptionSelection, TwoRoleOptionSelection;
    TwoRoleOutlineOption: AvailableTwoRoleOutlineOptionSelection, TwoRoleOutlineOptionSelection;
    String: AvailableStringSelection, StringSelection;
    Integer: AvailableIntegerSelection, IntegerSelection;
    Kira: AvailableKiraSelection, KiraSelection;
    ChatMessage: AvailableChatMessageSelection, ChatMessageSelection
}
