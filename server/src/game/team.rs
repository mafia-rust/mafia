use super::role::Role;


#[derive(Debug, PartialEq)]
pub enum Team{
    Faction, //eveyone within this faction is on a team
    Role
}
impl Team{
    pub fn same_team(a_role: Role, b_role: Role)->bool{
        let Some(a_team) = a_role.team() else {return false};
        let Some(b_team) = b_role.team() else {return false};
        if a_team != b_team {return false};

        match a_team {
            Team::Faction => a_role.faction_alignment().faction() == b_role.faction_alignment().faction(),
            Team::Role => a_role == b_role,
        }
    }
}