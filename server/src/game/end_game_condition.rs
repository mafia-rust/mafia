use super::{role_list::Faction, role::Role};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EndGameCondition {
    Town,
    Mafia,
    Cult,

    Arsonist,
    Werewolf,
    Ojo,

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
            EndGameCondition::Arsonist,
            EndGameCondition::Werewolf,
            EndGameCondition::Ojo,
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
            let mut all_agree = true;
            for living_role in &living_roles {
                if !EndGameCondition::keeps_game_running(*living_role) {continue;}
                if !EndGameCondition::required_conditions_for_win(*living_role).contains(&end_game_condition) {
                    all_agree = false;
                    break;
                }
            }
            if all_agree {
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
            Faction::Neutral => match role {
                Role::Minion => {
                    EndGameCondition::all().into_iter().filter(|end_game_condition|
                        match end_game_condition {
                            EndGameCondition::Town => false,
                            _ => true
                        }
                    ).collect()
                },

                Role::Arsonist => vec![EndGameCondition::Arsonist],
                Role::Werewolf => vec![EndGameCondition::Werewolf],
                Role::Ojo => vec![EndGameCondition::Ojo],

                Role::Death => vec![EndGameCondition::Death],
                
                Role::Politician => vec![EndGameCondition::Politician],
                _ => vec![]
            },
        }
    }
    ///Town, Mafia, Cult, NK
    pub fn keeps_game_running(role: Role)->bool{
        match role {
            Role::Jester | 
            Role::Hater |
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