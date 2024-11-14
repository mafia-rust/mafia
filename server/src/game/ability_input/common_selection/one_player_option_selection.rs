use serde::{Deserialize, Serialize};

use crate::{game::player::PlayerReference, vec_set::VecSet};

use super::AvailableSelection;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct OnePlayerOptionSelection(pub Option<PlayerReference>);
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableOnePlayerOptionSelection(pub VecSet<Option<PlayerReference>>);
impl AvailableSelection for AvailableOnePlayerOptionSelection{
    type Selection = OnePlayerOptionSelection;
    fn validate_selection(&self, selection: &OnePlayerOptionSelection)->bool{
        self.0.contains(&selection.0)
    }
}