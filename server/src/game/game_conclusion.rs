use serde::Serialize;

use super::{player::PlayerReference, role::Role, role_list::RoleSet, win_condition::WinCondition, Game};

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum GameConclusion {
    Town,
    Mafia,
    Cult,

    Fiends,

    Politician,

    Draw
}
impl GameConclusion {
    pub fn all()->Vec<GameConclusion>{
        vec![
            GameConclusion::Town,
            GameConclusion::Mafia,
            GameConclusion::Cult,
            GameConclusion::Fiends,
            GameConclusion::Politician,
            GameConclusion::Draw
        ]
    }
    ///either return Some(EndGameCondition) or None (if the game is not over yet)
    pub fn game_is_over(game: &Game)->Option<GameConclusion> {

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
            return Some(GameConclusion::Draw);
        }

        //find one end game condition that everyone agrees on
        for resolution in GameConclusion::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|p.keeps_game_running(game))
                    .all(|p|
                        match p.win_condition(game){
                            WinCondition::GameConclusionReached{win_if_any} => win_if_any.contains(&resolution),
                            WinCondition::RoleStateWon => true,
                        }
                    )
            {
                return Some(resolution);
            }
        }

        None
    }
    

    ///Town, Mafia, Cult, NK
    /// Is either town, or has the ability to consistently kill till the end of the game
    /// *has the ability to change what the set of living players win conditions are until game over (convert, marionette, kill)*
    /// A detective and a witch game never ends
    pub fn keeps_game_running(role: Role)->bool{

        match role {
            Role::Drunk => true,
            Role::Politician => true,
            _ => if 
                RoleSet::Neutral.get_roles().contains(&role) || 
                RoleSet::Minions.get_roles().contains(&role){
                false
            }else{
                true  
            }
        }


        
    }
}


//Endgamecondition -> One single game ending condition, if only these roles are left, the game ends
//Town, Mafia, Cult, Fiends, Politcian
//Victory condition -> If this is the endgamecondition of the game, you win
