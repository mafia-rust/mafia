use super::{role_list::Faction, role::Role};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EndGameCondition {
    Town,
    Mafia,
    Cult,

    Fiends,

    Death,
    Politician,

    Draw
}
impl EndGameCondition {
    pub fn all()->Vec<EndGameCondition>{
        vec![
            EndGameCondition::Town,
            EndGameCondition::Mafia,
            EndGameCondition::Cult,
            EndGameCondition::Fiends,
            EndGameCondition::Death,
            EndGameCondition::Politician
        ]
    }
    ///either return Some(EndGameCondition) or None (if the game is not over yet)
    pub fn game_is_over(living_roles: Vec<Role>) -> Option<EndGameCondition> {

        //Special wildcard case
        if living_roles.iter().all(|role|role==&Role::Wildcard)&&living_roles.len()>1{
            return None;
        }

        //if nobody is left to hold game hostage
        if !living_roles.iter().any(|role|EndGameCondition::keeps_game_running(*role)){
            return Some(EndGameCondition::Draw);
        }

        //find one end game condition that everyone agrees on
        for end_game_condition in EndGameCondition::all() {
            //if everyone who keeps the game running agrees on this end game condition, return it
            if
                living_roles.iter()
                    .filter(|r|EndGameCondition::keeps_game_running(**r))
                    .all(|r|EndGameCondition::required_conditions_for_win(*r).contains(&end_game_condition))
            {
                return Some(end_game_condition);
            }
        }

        None
    }
    pub fn can_win_together(a: Role, b: Role)->bool{
        let a_conditions = EndGameCondition::required_conditions_for_win(a);
        let b_conditions = EndGameCondition::required_conditions_for_win(b);

        if a_conditions.is_empty() || b_conditions.is_empty(){return true;}

        a_conditions.iter().any(|a_condition| b_conditions.contains(a_condition))
    }
    ///this role wins if the end game state is in this list
    pub fn required_conditions_for_win(role: Role)->Vec<EndGameCondition>{
        match role.faction(){
            Faction::Mafia => vec![EndGameCondition::Mafia],
            Faction::Cult => vec![EndGameCondition::Cult],
            Faction::Town => vec![EndGameCondition::Town],
            Faction::Fiends => vec![EndGameCondition::Fiends],
            Faction::Neutral => match role {
                Role::Minion => {
                    EndGameCondition::all().into_iter().filter(|end_game_condition|
                        match end_game_condition {
                            EndGameCondition::Town => false,
                            _ => true
                        }
                    ).collect()
                },
                Role::Politician => vec![EndGameCondition::Politician],
                _ => vec![]
            },
        }
    }
    ///Town, Mafia, Cult, NK
    pub fn keeps_game_running(role: Role)->bool{
        match role {
            Role::Jester | 
            Role::Provocateur |
            Role::Politician |
            Role::Death |
            Role::Minion |
            Role::Doomsayer |
            Role::Wildcard => false,
            _ => true
        }
    }
}


//Endgamecondition -> One single game ending condition, if only these roles are left, the game ends
//Town, Mafia, Cult, NK, Politcian
//Victory condition -> If this is the endgamecondition of the game, you win