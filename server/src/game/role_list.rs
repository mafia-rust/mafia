use std::vec;

use rand::Rng;
use serde::{Serialize, Deserialize};

use super::role::Role;

macro_rules! make_faction_enum {
    ($($name:ident),*)=>{
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Neutral
}
impl Faction{
    pub fn all_alignments(&self)->Vec<FactionAlignment>{
        match self{
            Faction::Town => vec![
                FactionAlignment::TownPower,
                FactionAlignment::TownInvestigative,
                FactionAlignment::TownProtective,
                FactionAlignment::TownKilling,
                FactionAlignment::TownSupport,
            ],
            Faction::Neutral => vec![
                FactionAlignment::NeutralEvil,
                FactionAlignment::NeutralKilling,
                FactionAlignment::NeutralChaos,
                FactionAlignment::NeutralApocalypse,
            ],
            Faction::Mafia => vec![
                FactionAlignment::MafiaKilling,
                FactionAlignment::MafiaDeception,
                FactionAlignment::MafiaSupport,
                FactionAlignment::MafiaPower,
            ],
        }
    }
}


macro_rules! make_faction_alignment_enum {
    ($($name:ident),*)=>{
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    MafiaPower,
    
    TownPower,
    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralChaos,
    NeutralApocalypse
}
impl FactionAlignment{
    pub fn faction(&self)->Faction{
        match self {
            Self::TownPower | Self::TownInvestigative | Self::TownProtective | Self::TownKilling | Self::TownSupport
                => Faction::Town,
            Self::NeutralEvil | Self::NeutralKilling | Self::NeutralChaos | Self::NeutralApocalypse
                => Faction::Neutral,
            Self::MafiaKilling | Self::MafiaDeception | Self::MafiaSupport | Self::MafiaPower
                => Faction::Mafia,
        }
    }
}

pub type RoleList = Vec<RoleOutline>;
pub fn create_random_roles(excluded_roles: &[RoleOutline], role_list: &RoleList) -> Option<Vec<Role>> {
    let mut taken_roles = Vec::new();
    for entry in role_list{
        if let Some(role) = entry.get_random_role(excluded_roles, &taken_roles){
            taken_roles.push(role);
        }else{
            return None;
        }
    }
    Some(taken_roles)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RoleOutline {
    #[serde(rename_all = "camelCase")]
    Exact{role: Role},
    #[serde(rename_all = "camelCase")]
    FactionAlignment{faction_alignment: FactionAlignment},
    #[serde(rename_all = "camelCase")]
    Faction{faction: Faction},
    Any
}

impl RoleOutline{
    pub fn get_all_possible_roles(&self, excluded_roles: &[RoleOutline], taken_roles: &[Role]) -> Vec<Role> {
        match self {
            RoleOutline::Exact { role } => vec![*role],
            RoleOutline::FactionAlignment { faction_alignment } => {
                Role::values().into_iter()
                    .filter(|r|r.faction_alignment() == *faction_alignment)
                    .filter(|r|!excluded_roles.contains(&RoleOutline::Exact { role: *r }))
                    .filter(|r|
                        match r.maximum_count() {
                            Some(m) => taken_roles.iter().filter(|r2|*r2==r).count() < m.into(),
                            None => true,
                        }
                    )
                    .collect()
            },
            RoleOutline::Faction { faction } => {
                Role::values().into_iter()
                    .filter(|r|r.faction_alignment().faction() == *faction)
                    .filter(|r|
                        !(
                            excluded_roles.contains(&RoleOutline::Exact { role: *r }) || 
                            excluded_roles.contains(&RoleOutline::FactionAlignment { faction_alignment: r.faction_alignment() })
                        )
                    ) 
                    .filter(|r|
                        match r.maximum_count() {
                            Some(m) => taken_roles.iter().filter(|r2|*r2==r).count() < m.into(),
                            None => true,
                        }

                    )
                    .collect()
            },
            RoleOutline::Any => {
                Role::values().into_iter()
                    .filter(|r|
                        !(
                            excluded_roles.contains(&RoleOutline::Exact { role: *r }) ||
                            excluded_roles.contains(&RoleOutline::FactionAlignment { faction_alignment: r.faction_alignment() }) ||
                            excluded_roles.contains(&RoleOutline::Faction { faction: r.faction_alignment().faction() })
                        )
                    )
                    .filter(|r|
                        match r.maximum_count() {
                            Some(m) => taken_roles.iter().filter(|r2|*r2==r).count() < m.into(),
                            None => true,
                        }
                        
                    )
                    .collect()
            },
        }
    }
    pub fn get_random_role(&self, excluded_roles: &[RoleOutline], taken_roles: &[Role]) -> Option<Role> {
        let possible_roles = self.get_all_possible_roles(excluded_roles, taken_roles);
        if possible_roles.is_empty() {return None;}
        let random_index = rand::thread_rng().gen_range(0..possible_roles.len());
        Some(possible_roles[random_index])
    }
}