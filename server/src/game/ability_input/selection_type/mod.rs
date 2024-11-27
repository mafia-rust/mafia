pub mod one_player_option_selection;
pub mod two_player_option_selection;
pub mod two_role_option_selection;
pub mod two_role_outline_option_selection;
pub mod role_option_selection;

use serde::{Deserialize, Serialize};

use super::{ability_selection::AbilitySelection, AbilityID, AbilityInput};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitSelection;



#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanSelection(pub bool);
impl AbilityInput{
    pub fn get_boolean_selection_if_id(&self, id: AbilityID)->Option<BooleanSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Boolean { selection } = self.selection() else {return None};
        Some(selection)
    }
}