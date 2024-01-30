use std::{collections::HashSet, vec};

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use vec1::Vec1;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleList(pub Vec<RoleOutline>);
impl RoleList {
    pub fn create_random_roles(&self, excluded_roles: &[Role]) -> Option<Vec<Role>> {
        let mut taken_roles = Vec::new();
        for entry in self.0.iter(){
            if let Some(role) = entry.get_random_role(excluded_roles, &taken_roles){
                taken_roles.push(role);
            }else{
                return None;
            }
        }
        Some(taken_roles)
    }
    pub fn simplify(&mut self){
        for entry in self.0.iter_mut(){
            entry.simplify();
        }
    }
    pub fn sort(&mut self){
        self.0.sort_by_key(|r| r.get_roles().len());
    }
}



#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RoleOutline{
    #[default]
    Any,
    RoleOutlineOptions{
        options: Vec1<RoleOutlineOption>
    },
}
impl RoleOutline{
    pub fn get_roles(&self) -> Vec<Role> {
        match self {
            RoleOutline::RoleOutlineOptions{options} => 
                options.iter().flat_map(|r| r.get_roles()).collect(),
            RoleOutline::Any => 
                Role::values(),
        }
    }
    pub fn get_random_role(&self, excluded_roles: &[Role], taken_roles: &[Role]) -> Option<Role> {
        let options = self.get_roles().into_iter().filter_taken_roles(excluded_roles, taken_roles).collect::<Vec<_>>();
        options.choose(&mut rand::thread_rng()).cloned()
    }
    pub fn simplify(&mut self){
        match self {
            RoleOutline::RoleOutlineOptions{options} => {
                let mut new_options = options.to_vec();

                new_options = new_options.into_iter().collect::<HashSet<_>>().into_iter().collect();

                for option_a in options.iter(){
                    for option_b in options.iter(){
                        if option_a.is_subset(option_b) && option_a != option_b{
                            new_options.retain(|r| r != option_a);
                        }
                    }
                }

                let mut new_options = Vec1::try_from_vec(new_options)
                    .expect("It is impossible to have two sets that are not equal but are subsets of eachother, role_list.rs: RoleOutline::simplify");

                new_options.sort();

                *self = RoleOutline::RoleOutlineOptions{options: new_options};
            }
            _ => {}
        }
    }
}



#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RoleOutlineOption {
    #[serde(rename_all = "camelCase")]
    RoleSet{role_set: RoleSet},
    #[serde(rename_all = "camelCase")]
    Role{role: Role},
    #[serde(rename_all = "camelCase")]
    Faction{faction: Faction}
}
impl RoleOutlineOption{
    pub fn get_roles(&self) -> Vec<Role> {
        match self {
            RoleOutlineOption::RoleSet { role_set } => {
                role_set.get_roles()
            }
            RoleOutlineOption::Role { role } => 
                vec![*role],
            RoleOutlineOption::Faction { faction } => 
                Role::values().into_iter().filter(|r|r.faction() == *faction).collect()
        }
    }
    pub fn is_subset(&self, other: &RoleOutlineOption) -> bool {
        self.get_roles().iter().all(|r|other.get_roles().contains(r))
    }
}
impl PartialOrd for RoleOutlineOption {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RoleOutlineOption {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.get_roles().len().cmp(&self.get_roles().len())
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoleSet {
    TownPower,
    TownSupport,
    TownKilling,
    TownProtective,
    TownInvestigative,
    TownCommon,

    MafiaPower,
    MafiaSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralChaos,
    NeutralApocalypse,
}
impl RoleSet{
    pub fn get_roles(&self) -> Vec<Role> {
        match self {
            RoleSet::TownPower => 
                vec![Role::Jailor, Role::Mayor],
            RoleSet::TownSupport => 
                vec![Role::Medium, Role::Retributionist, Role::Transporter, Role::Escort],
            RoleSet::TownKilling => 
                vec![Role::Vigilante, Role::Veteran, Role::Deputy],
            RoleSet::TownProtective => 
                vec![Role::Bodyguard, Role::Crusader, Role::Doctor, Role::Reveler],
            RoleSet::TownInvestigative => 
                vec![Role::Psychic, Role::Lookout, Role::Sheriff, Role::Spy, Role::Tracker, Role::Seer],
            RoleSet::TownCommon => 
                vec![].into_iter().chain(
                    RoleSet::TownInvestigative.get_roles().iter()
                ).chain(
                    RoleSet::TownProtective.get_roles().iter()
                ).chain(
                    RoleSet::TownKilling.get_roles().iter()
                ).chain(
                    RoleSet::TownSupport.get_roles().iter()
                ).cloned().collect(),
            RoleSet::MafiaPower => 
                vec![Role::Godfather, Role::Mafioso],
            RoleSet::MafiaSupport => 
                vec![
                    Role::Blackmailer, Role::Consigliere, Role::Consort, 
                    Role::Forger, Role::Framer, Role::Janitor, 
                    Role::Witch, Role::Necromancer
                ],
            RoleSet::NeutralEvil => 
                vec![Role::Jester, Role::Executioner, Role::Politician],
            RoleSet::NeutralKilling => 
                vec![Role::Arsonist, Role::Werewolf],
            RoleSet::NeutralChaos => 
                vec![Role::Vampire, Role::Amnesiac],
            RoleSet::NeutralApocalypse => 
                vec![Role::Death, Role::Doomsayer],
        }
    }
}





trait RoleIterator {
    fn filter_taken_roles(self, excluded_roles: &[Role], taken_roles: &[Role]) -> impl Iterator<Item = Role>;
}

impl<T: Iterator<Item = Role>> RoleIterator for T {
    fn filter_taken_roles(self, excluded_roles: &[Role], taken_roles: &[Role]) -> impl Iterator<Item = Role> {
        self
            .filter(|r|
                !excluded_roles.contains(r)
            )
            .filter(|r|
                match r.maximum_count() {
                    Some(max) => taken_roles.iter().filter(|r2|**r2==*r).count() < max.into(),
                    None => true,
                }
            )
    }
}