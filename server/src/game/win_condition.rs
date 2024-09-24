use std::collections::HashSet;

use super::resolution_state::ResolutionState;

/// Related functions require RoleStateWon to be independent of ResolutionState. 
/// RoleStateWon needs to be able to win with any ResolutionState.
#[derive(Debug, Clone)]
pub enum WinCondition{
    ResolutionStateReached{win_if_any: HashSet<ResolutionState>},
    RoleStateWon,
}



impl WinCondition{
    pub fn required_resolution_states_for_win(&self)->Option<HashSet<ResolutionState>>{
        match self{
            WinCondition::ResolutionStateReached{win_if_any} => Some(win_if_any.clone()),
            WinCondition::RoleStateWon => None,
        }
    }
    pub fn can_win_together(a: &WinCondition, b: &WinCondition)->bool{
        let a_conditions = a.required_resolution_states_for_win();
        let b_conditions = b.required_resolution_states_for_win();

        match (a_conditions, b_conditions){
            (Some(a), Some(b)) => a.intersection(&b).count() > 0,
            _ => true
        }
    }
    pub fn can_win_when_resolution_state_reached(&self, resolution_state: ResolutionState)->bool{
        match self{
            WinCondition::ResolutionStateReached{win_if_any} => win_if_any.contains(&resolution_state),
            WinCondition::RoleStateWon => true,
        }
    }
    pub fn requires_only_this_resolution_state(&self, resolution_state: ResolutionState)->bool{
        match self{
            WinCondition::ResolutionStateReached{win_if_any} => win_if_any.len() == 1 && win_if_any.contains(&resolution_state),
            WinCondition::RoleStateWon => false,
        }
    }
}