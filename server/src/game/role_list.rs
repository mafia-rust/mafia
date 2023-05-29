use std::vec;

use rand::Rng;
use serde::{Serialize, Deserialize};

use super::role::Role;

macro_rules! make_faction_enum {
    ($($name:ident),*)=>{
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Faction { $($name,)*}
        impl Faction {
            pub fn values() -> Vec<Self> {
                return vec![$(Self::$name),*];
            }
        }
    }
}
make_faction_enum!{
    Mafia,
    Town,
    Neutral,
    Coven
}
impl Faction{
    pub fn all_alignments(&self)->Vec<FactionAlignment>{
        match self{
            Faction::Coven => vec![
                FactionAlignment::CovenPower,
                FactionAlignment::CovenKilling,
                FactionAlignment::CovenUtility,
                FactionAlignment::CovenDeception
            ],
            Faction::Town => vec![
                FactionAlignment::TownPower,
                FactionAlignment::TownInvestigative,
                FactionAlignment::TownProtective,
                FactionAlignment::TownKilling,
                FactionAlignment::TownSupport
            ],
            Faction::Neutral => vec![
                FactionAlignment::NeutralEvil,
                FactionAlignment::NeutralKilling,
                FactionAlignment::NeutralChaos,
            ],
            Faction::Mafia => vec![
                FactionAlignment::MafiaKilling,
                FactionAlignment::MafiaDeception,
                FactionAlignment::MafiaSupport
            ],
        }
    }
    pub fn get_all_possible_faction_alignments(&self, taken_roles: &[Role])->Vec<FactionAlignment>{
        self.all_alignments().into_iter().filter(|potential_faction_alignment|{
            !potential_faction_alignment.get_all_possible_roles(taken_roles).is_empty()
        }).collect()
    }
    pub fn get_random_faction_alignment(&self, taken_roles: &[Role])->Option<FactionAlignment>{
        let possible_faction_alignments = self.get_all_possible_faction_alignments(taken_roles);
        if possible_faction_alignments.is_empty() {return None;}
        let random_index = rand::thread_rng().gen_range(0..possible_faction_alignments.len());
        Some(possible_faction_alignments[random_index].clone())
    }
}


macro_rules! make_faction_alignment_enum {
    ($($name:ident),*)=>{
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum FactionAlignment { $($name,)*}
        impl FactionAlignment {
            pub fn values() -> Vec<Self> {
                return vec![$(Self::$name),*];
            }
        }
    }
}
make_faction_alignment_enum!{
    MafiaKilling,
    MafiaDeception,
    MafiaSupport,

    TownPower,
    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralChaos,

    CovenPower,
    CovenKilling,
    CovenUtility,
    CovenDeception
}
impl FactionAlignment{
    pub fn faction(&self)->Faction{
        match self {
            Self::TownPower | Self::TownInvestigative | Self::TownProtective | Self::TownKilling | Self::TownSupport 
                => Faction::Town,
            Self::CovenPower |  Self::CovenKilling | Self::CovenUtility | Self::CovenDeception
                => Faction::Coven,
            Self::NeutralEvil | Self::NeutralKilling | Self::NeutralChaos 
                => Faction::Neutral,
            Self::MafiaKilling | Self::MafiaDeception | Self::MafiaSupport 
                => Faction::Mafia,
        }
    }
    pub fn get_all_possible_roles(&self, taken_roles: &[Role])->Vec<Role>{
        Role::values().into_iter().filter(|potential_role|{
            if potential_role.faction_alignment() != *self {return false;}
            let Some(potantial_role_max_count) = potential_role.maximum_count() else {return true};
            
            taken_roles.iter().filter(|taken_role|{
                **taken_role == *potential_role
            }).count() < potantial_role_max_count.into()
        }).collect()
    }
    pub fn get_random_role(&self, taken_roles: &[Role])->Option<Role>{
        let possible_roles = self.get_all_possible_roles(taken_roles);
        if possible_roles.is_empty() {return None;}
        let random_index = rand::thread_rng().gen_range(0..possible_roles.len());
        Some(possible_roles[random_index])
    }
}

pub type RoleList = Vec<RoleListEntry>;
pub fn create_random_roles(role_list: &RoleList) -> Vec<Role> {
    let mut taken_roles = Vec::new();
    for entry in role_list{
        taken_roles.push(entry.get_random_role(&taken_roles));
    }
    taken_roles
}
pub fn get_all_possible_roles(role_list: &RoleList) -> Vec<Role> {
    //if executioner then add jester
    //if there could be mafioso at beginning then add godfather
    //if any mafia(besides godfather) then add mafioso
    todo!()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RoleListEntry {
    #[serde(rename_all = "camelCase")]
    Exact{role: Role},
    #[serde(rename_all = "camelCase")]
    FactionAlignment{faction_alignment: FactionAlignment},
    #[serde(rename_all = "camelCase")]
    Faction{faction: Faction},
    Any
}

impl RoleListEntry{
    pub fn get_random_role(&self, taken_roles: &[Role]) -> Role {
        match self {
            RoleListEntry::Exact { role } => *role,
            RoleListEntry::FactionAlignment { faction_alignment } => {
                if let Some(role) = faction_alignment.get_random_role(taken_roles){
                    role
                } else {
                    RoleListEntry::Faction { faction: faction_alignment.faction() }.get_random_role(taken_roles)
                }
            },
            RoleListEntry::Faction { faction } => {
                if let Some(faction_alignment) = faction.get_random_faction_alignment(taken_roles){
                    faction_alignment.get_random_role(taken_roles).expect("just checked that there was an available role")
                } else {
                    RoleListEntry::Any.get_random_role(taken_roles)
                }
            },
            RoleListEntry::Any => {
                let mut all_factions = Faction::values().into_iter().filter(|faction|{
                    faction.get_random_faction_alignment(taken_roles).is_some()
                }).collect::<Vec<Faction>>();

                if all_factions.is_empty() {
                    all_factions = Faction::values();
                }

                let random_faction = all_factions.get(
                    rand::thread_rng().gen_range(0..all_factions.len())).expect("there should be at least one role");
                RoleListEntry::Faction{faction: random_faction.clone()}.get_random_role(taken_roles)
            },
        }
    }


    pub fn get_possible_roles(&self) -> Vec<Role> {
        match self {
            RoleListEntry::Exact{role}=> 
                vec![*role],
            RoleListEntry::FactionAlignment{faction_alignment}=> 
                Role::values().into_iter().filter(|role|{
                    role.faction_alignment() == *faction_alignment
                }).collect(),
            RoleListEntry::Faction{faction}=>
                Role::values().into_iter().filter(|role|{
                    role.faction_alignment().faction() == *faction
                }).collect(),
            RoleListEntry::Any => Role::values(),
        }
    }
    pub fn get_possible_roles_given_taken_roles(&self, taken_roles: &[Role]) -> Vec<Role> {
        let possible_roles = self.get_possible_roles();
        possible_roles.into_iter().filter(|potential_role|{
            let Some(potantial_role_max_count) = potential_role.maximum_count() else {return true};
            
            (taken_roles.iter().filter(|taken_role|{
                *taken_role == potential_role
            }).count() < potantial_role_max_count.into())
        }).collect()
    }
}