use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::{ability_selection::AbilitySelection, AbilityID, AbilityInput, ValidateAvailableSelection}, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePlayerOptionSelection(pub Option<PlayerReference>);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableOnePlayerOptionSelection(pub VecSet<Option<PlayerReference>>);
impl ValidateAvailableSelection for AvailableOnePlayerOptionSelection{
    type Selection = OnePlayerOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &OnePlayerOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}


impl AbilityInput{
    pub fn get_player_option_selection_if_id(&self, id: AbilityID)->Option<OnePlayerOptionSelection>{
        if id != self.id() {return None};
        let AbilitySelection::OnePlayerOption { selection } = self.selection() else {return None};
        Some(selection)
    }
}