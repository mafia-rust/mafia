use std::collections::HashSet;

use super::{role_list::Faction, role::Role};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GameOverState {
    Town,
    Mafia,
    Cult,

    Fiends,

    Death,
    Politician,

    Draw
}
impl GameOverState {
    pub fn all()->Vec<GameOverState>{
        vec![
            GameOverState::Town,
            GameOverState::Mafia,
            GameOverState::Cult,
            GameOverState::Fiends,
            GameOverState::Death,
            GameOverState::Politician
        ]
    }
    ///either return Some(EndGameCondition) or None (if the game is not over yet)
    pub fn game_is_over(living_roles: Vec<Role>) -> Option<GameOverState> {

        //Special wildcard case
        if living_roles.iter().all(|role|
            matches!(role, Role::Wildcard|Role::TrueWildcard)
        )&&living_roles.len()>1{
            return None;
        }

        //if nobody is left to hold game hostage
        if !living_roles.iter().any(|role|GameOverState::keeps_game_running(*role)){
            return Some(GameOverState::Draw);
        }

        //find one end game condition that everyone agrees on
        for end_game_condition in GameOverState::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                living_roles.iter()
                    .filter(|r|GameOverState::keeps_game_running(**r))
                    .all(|r|GameOverState::required_conditions_for_win(*r).contains(&end_game_condition))
            {
                return Some(end_game_condition);
            }
        }

        None
    }
    pub fn can_win_together(a: Role, b: Role)->bool{
        let a_conditions = GameOverState::required_conditions_for_win(a);
        let b_conditions = GameOverState::required_conditions_for_win(b);

        if a_conditions.is_empty() || b_conditions.is_empty(){return true;}

        a_conditions.iter().any(|a_condition| b_conditions.contains(a_condition))
    }
    ///this role wins if the end game state is in this list
    pub fn required_conditions_for_win(role: Role)->HashSet<GameOverState>{
        match role.faction(){
            Faction::Mafia => vec![GameOverState::Mafia],
            Faction::Cult => vec![GameOverState::Cult],
            Faction::Town => vec![GameOverState::Town],
            Faction::Fiends => vec![GameOverState::Fiends],
            Faction::Neutral => match role {
                Role::Minion => {
                    GameOverState::all().into_iter().filter(|end_game_condition|
                        match end_game_condition {
                            GameOverState::Town => false,
                            _ => true
                        }
                    ).collect()
                },
                Role::Politician => vec![GameOverState::Politician],
                _ => vec![]
            },
        }.into_iter().collect()
    }
    ///Town, Mafia, Cult, NK
    pub fn keeps_game_running(role: Role)->bool{
        if role.faction() == Faction::Neutral{
            false
        }else{
            true
        }
    }
}


//Endgamecondition -> One single game ending condition, if only these roles are left, the game ends
//Town, Mafia, Cult, NK, Politcian
//Victory condition -> If this is the endgamecondition of the game, you win