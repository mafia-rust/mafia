use super::{role_list::Faction, role::Role};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EndGameCondition {
    Faction(Faction),
    Role(Role),

    None
}
impl EndGameCondition {
    pub fn can_win_together(a: EndGameCondition, b: EndGameCondition)->bool{
        a == Self::None || b == Self::None || a == b
    }
    pub fn from_role(role: Role) -> EndGameCondition {
        match role.faction(){
            Faction::Mafia => EndGameCondition::Faction(Faction::Mafia),
            Faction::Cultist => EndGameCondition::Faction(Faction::Cultist),
            Faction::Town => EndGameCondition::Faction(Faction::Town),
            Faction::Neutral => match role {
                Role::Jester | Role::Executioner | Role::Politician |
                Role::Doomsayer | Role::Death |
                Role::Amnesiac => EndGameCondition::None,
                _ => EndGameCondition::Role(role)
            },
        }
    }
}