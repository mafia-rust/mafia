use super::{player::PlayerReference, role::Role, role_list::Faction, win_condition::WinCondition, Game};

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
            ResolutionState::Politician,
            ResolutionState::Draw
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
        for resolution in ResolutionState::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|p.keeps_game_running(game))
                    .all(|p|
                        match p.win_condition(game){
                            WinCondition::ResolutionStateReached{win_if_any} => win_if_any.contains(&resolution),
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
