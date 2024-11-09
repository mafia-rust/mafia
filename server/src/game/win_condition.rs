use std::collections::HashSet;

use super::game_conclusion::GameConclusion;

/// Related functions require RoleStateWon to be independent of GameConclusion. 
/// RoleStateWon needs to be able to win with any GameConclusion.
#[derive(Debug, Clone)]
pub enum WinCondition{
    GameConclusionReached{win_if_any: HashSet<GameConclusion>},
    RoleStateWon,
}



impl WinCondition{
    pub fn required_resolution_states_for_win(&self)->Option<HashSet<GameConclusion>>{
        match self{
            WinCondition::GameConclusionReached{win_if_any} => Some(win_if_any.clone()),
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
    pub fn can_win_when_resolution_state_reached(&self, resolution_state: GameConclusion)->bool{
        match self{
            WinCondition::GameConclusionReached{win_if_any} => win_if_any.contains(&resolution_state),
            WinCondition::RoleStateWon => true,
        }
    }
    pub fn is_loyalist_for(&self, resolution_state: GameConclusion)->bool{
        match self{
            WinCondition::GameConclusionReached{win_if_any} => win_if_any.len() == 1 && win_if_any.contains(&resolution_state),
            WinCondition::RoleStateWon => false,
        }
    }
    
    pub fn new_single_resolution_state(resolution_state: GameConclusion) -> WinCondition {
        let mut win_if_any = HashSet::new();
        win_if_any.insert(resolution_state);
        WinCondition::GameConclusionReached { win_if_any }
    }
}