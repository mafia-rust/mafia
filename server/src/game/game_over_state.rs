use std::collections::HashSet;

use super::{player::PlayerReference, role::Role, role_list::Faction, Game};

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
    pub fn game_is_over(game: &Game)->Option<GameOverState> {

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
            return Some(GameOverState::Draw);
        }

        //find one end game condition that everyone agrees on
        for end_game_condition in GameOverState::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|p.keeps_game_running(game))
                    .all(|p|
                        if let Some(set) = p.required_game_over_states_for_win(game){
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
    
    pub fn can_win_with(game: &Game, player: PlayerReference, game_over_state: GameOverState)->bool{
        if let Some(set) = player.required_game_over_states_for_win(game){
            set.contains(&game_over_state)
        }else{
            true
        }
        
    }
    pub fn exclusively_wins_with(game: &Game, player: PlayerReference, game_over_state: GameOverState)->bool{
        if let Some(set) = player.required_game_over_states_for_win(game) {
            set.len() == 1 && set.contains(&game_over_state)
        }else{
            false
        }
    }
    
    pub fn can_win_together(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let a_conditions = a.required_game_over_states_for_win(game);
        let b_conditions = b.required_game_over_states_for_win(game);

        match (a_conditions, b_conditions){
            (Some(a), Some(b)) => a.intersection(&b).count() > 0,
            _ => true
        }
    }
    

    ///this role wins if the end game state is in this list
    pub fn required_game_over_states_for_win(role: Role)->Option<HashSet<GameOverState>>{
        Some(match role.faction(){
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
                _ => {return None;}
            },
        }.into_iter().collect())
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
//Town, Mafia, Cult, Fiends, Politcian
//Victory condition -> If this is the endgamecondition of the game, you win
