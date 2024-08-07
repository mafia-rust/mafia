use std::collections::HashSet;

use super::{player::PlayerReference, role::Role, role_list::Faction, Game};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ResolutionState {
    Town,
    Mafia,
    Cult,

    Fiends,

    Death,
    Politician,

    Draw
}
impl ResolutionState {
    pub fn all()->Vec<ResolutionState>{
        vec![
            ResolutionState::Town,
            ResolutionState::Mafia,
            ResolutionState::Cult,
            ResolutionState::Fiends,
            ResolutionState::Death,
            ResolutionState::Politician
        ]
    }
    ///either return Some(EndGameCondition) or None (if the game is not over yet)
    pub fn game_is_over(game: &Game)->Option<ResolutionState> {

        //Special wildcard case
        let living_roles = PlayerReference::all_players(game).filter_map(|player|{
            if player.alive(game){
                Some(player.role(game))
            }else{
                None
            }
        }).collect::<Vec<_>>();

        if living_roles.iter().all(|role|matches!(role, Role::Wildcard|Role::TrueWildcard)) && living_roles.len() > 1 {
            return None;
        }
        
        //if nobody is left to hold game hostage
        if !PlayerReference::all_players(game).any(|player|player.keeps_game_running(game)){
            return Some(ResolutionState::Draw);
        }

        //find one end game condition that everyone agrees on
        for end_game_condition in ResolutionState::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|p.keeps_game_running(game))
                    .all(|p|
                        if let Some(set) = p.required_resolution_states_for_win(game){
                            set.contains(&end_game_condition)
                        }else{
                            true
                        }
                    )
            {
                return Some(end_game_condition);
            }
        }

        None
    }
    
    pub fn can_win_with(game: &Game, player: PlayerReference, resolution_state: ResolutionState)->bool{
        if let Some(set) = player.required_resolution_states_for_win(game){
            set.contains(&resolution_state)
        }else{
            true
        }
        
    }
    pub fn requires_only_this_resolution_state(game: &Game, player: PlayerReference, resolution_state: ResolutionState)->bool{
        if let Some(set) = player.required_resolution_states_for_win(game) {
            set.len() == 1 && set.contains(&resolution_state)
        }else{
            false
        }
    }
    
    pub fn can_win_together(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let a_conditions = a.required_resolution_states_for_win(game);
        let b_conditions = b.required_resolution_states_for_win(game);

        match (a_conditions, b_conditions){
            (Some(a), Some(b)) => a.intersection(&b).count() > 0,
            _ => true
        }
    }
    

    ///this role wins if the end game state is in this list
    pub fn required_resolution_states_for_win(role: Role)->Option<HashSet<ResolutionState>>{
        Some(match role.faction(){
            Faction::Mafia => vec![ResolutionState::Mafia],
            Faction::Cult => vec![ResolutionState::Cult],
            Faction::Town => vec![ResolutionState::Town],
            Faction::Fiends => vec![ResolutionState::Fiends],
            Faction::Neutral => match role {
                Role::Minion | Role::Scarecrow => {
                    ResolutionState::all().into_iter().filter(|end_game_condition|
                        match end_game_condition {
                            ResolutionState::Town => false,
                            _ => true
                        }
                    ).collect()
                },
                Role::Politician => vec![ResolutionState::Politician],
                _ => {return None;}
            },
        }.into_iter().collect())
    }
    ///Town, Mafia, Cult, NK
    /// Is either town, or has the ability to consistently kill till the end of the game
    /// *has the ability to change what the set of living players win conditions are until game over (convert, marionette, kill)*
    /// A detective and a minion game never ends
    pub fn keeps_game_running(role: Role)->bool{
        if role.faction() == Faction::Neutral{
            false
        }else{
            true  
        }
    }
}


//Endgamecondition -> One single game ending condition, if only these roles are left, the game ends
//Town, Mafia, Cult, Fiends, Politcian
//Victory condition -> If this is the endgamecondition of the game, you win
