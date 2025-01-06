use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::ValidateAvailableSelection, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreePlayerOptionSelection(pub Option<PlayerReference>, pub Option<PlayerReference>, pub Option<PlayerReference>);
impl ThreePlayerOptionSelection{
    pub fn any_in_common(&self, other: &ThreePlayerOptionSelection) -> bool{
        vec![self.0, self.1, self.2].iter().any(|player|
            if let Some(player) = player{
                other.contains(*player)
            }else{
                false
            }
        )
    }
    pub fn same_role(&self) -> bool{
        (self.0.is_some() && self.0 == self.1) ||
        (self.0.is_some() && self.0 == self.2) ||
        (self.1.is_some() && self.1 == self.2)
    }
    pub fn contains(&self, player: PlayerReference) -> bool{
        self.0 == Some(player) || self.1 == Some(player) || self.2 == Some(player)
    }
    pub fn any_is_some(&self) -> bool{
        self.0.is_some() || self.1.is_some() || self.2.is_some()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableThreePlayerOptionSelection{
    pub available_first_players: VecSet<Option<PlayerReference>>,
    pub available_second_players: VecSet<Option<PlayerReference>>,
    pub available_third_players: VecSet<Option<PlayerReference>>,
    
    pub can_choose_duplicates: bool
}
impl PartialOrd for AvailableThreePlayerOptionSelection{
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering>{
        Some(Ordering::Equal)
    }
}
impl Ord for AvailableThreePlayerOptionSelection{
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering{
        Ordering::Equal
    }
}
impl ValidateAvailableSelection for AvailableThreePlayerOptionSelection{
    type Selection = ThreePlayerOptionSelection;
    fn validate_selection(&self, _game: &Game, selection: &ThreePlayerOptionSelection)->bool{
        if !self.can_choose_duplicates && selection.same_role(){
            return false
        }
        self.available_first_players.contains(&selection.0) && self.available_first_players.contains(&selection.1)
    }
}