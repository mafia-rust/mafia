pub mod two_player_option_selection; pub use two_player_option_selection::*;
pub mod two_role_option_selection; pub use two_role_option_selection::*;
pub mod two_role_outline_option_selection; pub use two_role_outline_option_selection::*;
pub mod three_player_option_selection; pub use three_player_option_selection::*;
pub mod role_option_selection; pub use role_option_selection::*;
pub mod kira_selection; pub use kira_selection::*;
pub mod player_list_selection; pub use player_list_selection::*;

use serde::{Deserialize, Serialize};

use super::{ability_selection::AbilitySelection, ControllerID, AbilityInput};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitSelection;



#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanSelection(pub bool);
impl AbilityInput{
    pub fn get_boolean_selection_if_id(&self, id: ControllerID)->Option<BooleanSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Boolean { selection } = self.selection() else {return None};
        Some(selection)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringSelection(pub String);
impl AbilityInput{
    pub fn get_string_selection_if_id(&self, id: ControllerID)->Option<BooleanSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Boolean { selection } = self.selection() else {return None};
        Some(selection)
    }
}