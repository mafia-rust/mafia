use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::ValidateAvailableSelection, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPlayerOptionSelection(pub Option<PlayerReference>, pub Option<PlayerReference>);
impl TwoPlayerOptionSelection{
    pub fn any_in_common(&self, other: &TwoPlayerOptionSelection) -> bool{
        (self.0.is_some() && self.0 == other.0) || 
        (self.0.is_some() && self.0 == other.1) || 
        (self.1.is_some() && self.1 == other.0) || 
        (self.1.is_some() && self.1 == other.1)
    }
    pub fn same_role(&self) -> bool{
        self.0.is_some() && self.0 == self.1 
    }
    pub fn contains(&self, player: PlayerReference) -> bool{
        self.0 == Some(player) || self.1 == Some(player)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTwoPlayerOptionSelection{
    pub available_first_player: VecSet<Option<PlayerReference>>,
    pub available_second_player: VecSet<Option<PlayerReference>>,
    
    pub can_choose_duplicates: bool
}
impl PartialOrd for AvailableTwoPlayerOptionSelection{
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering>{
        Some(Ordering::Equal)
    }
}
impl Ord for AvailableTwoPlayerOptionSelection{
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering{
        Ordering::Equal
    }
}
impl ValidateAvailableSelection for AvailableTwoPlayerOptionSelection{
    type Selection = TwoPlayerOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &TwoPlayerOptionSelection)->bool{
        if !self.can_choose_duplicates && selection.same_role(){
            return false
        }
        self.available_first_player.contains(&selection.0) && self.available_first_player.contains(&selection.1)
    }
}