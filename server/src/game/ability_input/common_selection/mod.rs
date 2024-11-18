pub mod one_player_option_selection;
pub mod two_player_option_selection;
pub mod two_role_option_selection;
pub mod two_role_outline_option_selection;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitSelection;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanSelection(pub bool);



pub trait AvailableSelection{
    type Selection;
    fn validate_selection(&self, selection: &Self::Selection)->bool;
}





