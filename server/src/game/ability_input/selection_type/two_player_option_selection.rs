use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::ValidateAvailableSelection, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPlayerOptionSelection(pub Option<(PlayerReference, PlayerReference)>);
impl TwoPlayerOptionSelection{
    pub fn any_in_common(&self, other: &TwoPlayerOptionSelection) -> bool{
        match (self.0, other.0) {
            (Some((first, second)), Some((other_first, other_second))) => {
                first == other_first || 
                first == other_second || 
                second == other_first || 
                second == other_second
            },
            _ => false
        }
    }
    pub fn same_role(&self) -> bool{
        if let Some((first, second)) = self.0{
            first == second
        }else{
            false
        }
    }
    pub fn contains(&self, player: PlayerReference) -> bool{
        if let Some((first, second)) = self.0{
            first == player || second == player
        }else{
            false
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTwoPlayerOptionSelection{
    pub available_first_players: VecSet<PlayerReference>,
    pub available_second_players: VecSet<PlayerReference>,
    
    pub can_choose_duplicates: bool,
    pub can_choose_none: bool
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
        let Some((first, second)) = selection.0 else {
            if self.can_choose_none{
                return true
            }else{
                return false
            }
        };

        if !self.can_choose_duplicates && first == second{
            return false
        }

        if 
            !self.available_first_players.contains(&first) || 
            !self.available_second_players.contains(&second)
        {
            return false
        }
        
        true
    }
}