use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::ValidateAvailableSelection, player::PlayerReference}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePlayerOptionSelection(pub Option<PlayerReference>);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableOnePlayerOptionSelection(pub VecSet<Option<PlayerReference>>);
impl ValidateAvailableSelection for AvailableOnePlayerOptionSelection{
    type Selection = OnePlayerOptionSelection;
    fn validate_selection(&self, selection: &OnePlayerOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}