use super::role::Role;

pub enum Faction{
    Mafia,
    Town,
    Neutral,
    Coven,
}
impl Faction{
    pub fn all_alignments(&self)->Vec<FactionAlignment>{
        match self{
            Faction::Mafia => vec![
                FactionAlignment::MafiaKilling,
                FactionAlignment::MafiaDeception,
                FactionAlignment::MafiaSupport
            ],
            Faction::Town => vec![
                FactionAlignment::TownInvestigative,
                FactionAlignment::TownProtective,
                FactionAlignment::TownKilling,
                FactionAlignment::TownSupport
            ],
            Faction::Neutral => vec![
                FactionAlignment::NeutralEvil,
                FactionAlignment::NeutralKilling,
                FactionAlignment::NeutralChaos,
                FactionAlignment::NeutralBenign,
            ],
            Faction::Coven => vec![
                FactionAlignment::CovenEvil
            ],
        }
    }
}
pub enum FactionAlignment{
    MafiaKilling,
    MafiaDeception,
    MafiaSupport,

    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralBenign,
    NeutralChaos,

    CovenEvil
}
impl FactionAlignment{
    pub fn faction(&self)->Faction{
        match self {
            Self::MafiaKilling | Self::MafiaDeception | Self::MafiaSupport => Faction::Mafia,
            Self::TownInvestigative | Self::TownProtective | Self::TownKilling | Self::TownSupport => Faction::Town,
            Self::NeutralEvil | Self::NeutralKilling | Self::NeutralBenign | Self::NeutralChaos => Faction::Neutral,
            Self::CovenEvil => Faction::Coven,
        }
    }
}

pub struct RoleList{
    role_list: Vec<RoleListEntry>
}
impl RoleList{
    // pub fn create_random_roles(&self)->Vec<Role>{
    //     //length of out vec will be same as in vec
    // }
    // pub fn get_all_possible_roles(&self)->Vec<Role>{
    //     //if executioner then add jester
    //     //if there could be mafioso at beginning then add godfather
    //     //if any mafia(besides godfather) then add mafioso

    // }
}
pub enum RoleListEntry{
    Exact(Role),
    FactionAlignment(FactionAlignment),
    Faction(Faction),
    Any
}
impl RoleListEntry{
    // pub fn get_random_role()->Role{
        
    // }
    // pub fn get_possible_roles()->Role{

    // }
}