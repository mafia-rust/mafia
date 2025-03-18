use std::ops::AddAssign;

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use vec1::{
    vec1,
    Vec1
};

use crate::vec_set::VecSet;

use super::{components::insider_group::InsiderGroupID, game_conclusion::GameConclusion, role::Role};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleList(pub Vec<RoleOutline>);

impl RoleList {
    /// Output is the same order as the rolelist
    pub fn simplify(&mut self){
        for entry in self.0.iter_mut(){
            entry.simplify();
        }
    }
    pub fn sort(&mut self){
        self.0.sort_by_key(|r| r.get_role_assignments().len());
    }
    /// Output is the same order as the rolelist
    pub fn generator(&self, enabled_roles: &VecSet<Role>) -> Result<RoleAssignmentGen, ()>{
        let mut taken_roles = Vec::new();
        let mut outline_gens = Vec::new();
        for outline in self.0.iter() {
            let gen = outline.generator(enabled_roles);
            if gen.0.is_empty() {return Err(())};
            outline_gens.push(gen.clone());
            if gen.0.len() > 1 {continue}
            let Some(option) = gen.0.first() else {return Err(())};
            if option.role.role_limit_1() {
                taken_roles.push(option.role);
            }
        }
        let new_taken_roles = &mut Vec::new();
        loop {
            for i in 0..outline_gens.len() {
                let outline = &mut outline_gens[i];
                if outline.0.len() == 1 {continue;}
                outline.0.retain(|option|!taken_roles.contains(&option.role));
                if outline.0.len() == 0 {return Err(())}
                if outline.0.len() == 1 {
                    if outline.0[0].role.role_limit_1() {
                        taken_roles.push(outline.0[0].role);
                    }
                }
            }

            if new_taken_roles.is_empty() {break}

            taken_roles = new_taken_roles.clone();
            new_taken_roles.clear();
        }
        //using new taken roles because its guaranteed to be empty and it doesn't need to reallocate
        for outline in outline_gens.iter() {
            if outline.0.len() > 1 {continue;}
            let Some(role_assignment) = outline.0.first() else {return Err(())};
            if !role_assignment.role.role_limit_1() {continue}
            if new_taken_roles.contains(&role_assignment.role) {return Err(())};
            new_taken_roles.push(role_assignment.role);
        }
        
        return Ok(RoleAssignmentGen(outline_gens));
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoleAssignmentGen(pub Vec<RoleOutlineGenData>);
impl RoleAssignmentGen {
    pub fn create_random_role_assignments(&self) -> Result<Vec<RoleAssignment>, ()> {
        let mut generated_data = Vec::<RoleAssignment>::new();
        let rng = &mut rand::rng();
        for entry in self.0.iter(){
            if let Some(assignment) = entry.0.choose(rng){
                let role = assignment.role;
                match role.maximum_count() {
                    Some(max) => {
                        //Makes sure it doesn't go over the role limit
                        //starts at 1 because the current data has not been pushed 
                        let count = &mut 1;
                        if generated_data.iter().any(|data| 
                            if data.role == role {
                                if *count == max {
                                    true
                                } else {
                                    count.add_assign(1);
                                    false
                                }
                            } else {
                                false
                            }
                        ) {
                            return Err(());
                        }
                    }
                    _=>(),
                }
                generated_data.push(assignment.clone());
            } else {
                return Err(());
            }
        }
        Ok(generated_data)
    }
    ///Returns true if the role is guaranteed to exist with the default win condition and insider group
    pub fn specifies_role_with_defaults(&self, role: Role) -> bool {
        self.0.contains(&RoleOutlineGenData(vec![RoleAssignment::new_from_default(role)]))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct RoleAssignment {
    pub role: Role,
    pub insider_groups: RoleOutlineOptionInsiderGroups,
    pub win_condition: RoleOutlineOptionWinCondition
}
impl RoleAssignment {
    pub fn new_from_default(role: Role)->RoleAssignment{
        RoleAssignment{
            role,
            win_condition: RoleOutlineOptionWinCondition::default(),
            insider_groups: RoleOutlineOptionInsiderGroups::default(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOutline {
    pub options: Vec1<RoleOutlineOption>
}

impl Serialize for RoleOutline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.options.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for RoleOutline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Self {
            options: Vec1::<RoleOutlineOption>::deserialize(deserializer)?
        })
    }
}

impl Default for RoleOutline {
    fn default() -> Self {
        Self {options: vec1![RoleOutlineOption{
            win_condition: Default::default(),
            insider_groups: Default::default(),
            roles: RoleOutlineOptionRoles::RoleSet { role_set: RoleSet::Any }
        }]}
    }
}
impl RoleOutline{
    pub fn new_exact(role: Role)->RoleOutline{
        RoleOutline{options: vec1![RoleOutlineOption{
            win_condition: Default::default(),
            insider_groups: Default::default(),
            roles: RoleOutlineOptionRoles::Role{role}
        }]}
    }
    pub fn get_role_assignments(&self) -> Vec<RoleAssignment> {
        self.options.iter()
            .flat_map(|r| 
                r.roles.get_roles().into_iter()
                    .map(|role| RoleAssignment{
                        role,
                        insider_groups: r.insider_groups.clone(),
                        win_condition: r.win_condition.clone()
                    })
            ).collect()
    }
    pub fn get_random_role_assignment(&self, enabled_roles: &VecSet<Role>, taken_roles: &[Role]) -> Option<RoleAssignment> {
        self.get_role_assignments().iter()
            .filter(|option|
                role_can_generate(option.role, enabled_roles, taken_roles)
            ).collect::<Vec<&RoleAssignment>>()
            .choose(&mut rand::rng()).cloned().cloned()
    }
    pub fn simplify(&mut self){
        let mut new_options = self.options.to_vec();

        new_options = new_options.into_iter().collect::<VecSet<_>>().into_iter().collect();

        for option_a in self.options.iter(){
            if !option_a.roles.is_role_set() {continue};
            for option_b in self.options.iter(){
                if option_a != option_b && option_b.subset_of(&option_a) {
                    new_options.retain(|r| r != option_a);
                }
            }
        }

        let mut new_options = Vec1::try_from_vec(new_options)
            .expect("It is impossible to have two sets that are not equal but are subsets of each other, role_list.rs: RoleOutline::simplify");

        new_options.sort();

        *self = RoleOutline{options: new_options};
    }
    pub fn simplified(&self) -> Self{
        let mut clone = self.clone();
        clone.simplify();
        return clone;
    }
    /// Reduces the number of roles that can generate from this to a value closer to the actual possible number
    /// This
    /// - Speeds up picking roles from outlines
    /// - Allows for more fail-fast cases when trying to start game with a role list that cannot generate
    /// - Makes the probability of picking any given combination of roles more uniform
    /// - Reduces the probability of failing even when there is a valid way to generate roles from the role list
    pub fn generator(&self, enabled_roles:&VecSet<Role>) -> RoleOutlineGenData {
        let options = self.simplified().options;
        let mut new_options = VecSet::with_capacity(options.len());
        for option in options {
            let opt_gen = option.generator(enabled_roles);
            new_options = new_options.union(&opt_gen);
        };
        let new_options: Vec<RoleAssignment> = new_options.into_iter().collect();
        RoleOutlineGenData(new_options)
    }
}
//Don't be stupid, make sure the game doesn't start if there isn't anything in the outline
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoleOutlineGenData(pub Vec<RoleAssignment>);

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, PartialOrd, Ord)]
#[serde(untagged, rename_all = "camelCase")]
pub enum RoleOutlineOptionWinCondition {
    #[default] RoleDefault,
    #[serde(rename_all = "camelCase")]
    GameConclusionReached { win_if_any: VecSet<GameConclusion> },
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, PartialOrd, Ord)]
#[serde(untagged, rename_all = "camelCase")]
pub enum RoleOutlineOptionInsiderGroups {
    #[default] RoleDefault,
    #[serde(rename_all = "camelCase")]
    Custom { insider_groups: VecSet<InsiderGroupID> },
}

impl RoleOutlineOptionWinCondition {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::RoleDefault)
    }
}

impl RoleOutlineOptionInsiderGroups {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::RoleDefault)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RoleOutlineOption {
    #[serde(flatten)]
    pub roles: RoleOutlineOptionRoles,
    #[serde(flatten, skip_serializing_if = "RoleOutlineOptionWinCondition::is_default")]
    pub win_condition: RoleOutlineOptionWinCondition,
    #[serde(flatten, skip_serializing_if = "RoleOutlineOptionInsiderGroups::is_default")]
    pub insider_groups: RoleOutlineOptionInsiderGroups,
}

/// Watch this!
impl<'de> Deserialize<'de> for RoleOutlineOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        let mut option = RoleOutlineOption::default();
        
        let json = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::Object(map) = json {
            if let Some(value) = map.get("winIfAny") {
                if let Ok(string_win_condition) = serde_json::to_string(value) {
                    if let Ok(win_if_any) = serde_json::from_str(string_win_condition.as_str()) {
                        option.win_condition = RoleOutlineOptionWinCondition::GameConclusionReached { win_if_any}
                    }
                }
            }
            if let Some(value) = map.get("insiderGroups") {
                if let Ok(string_insider_groups) = serde_json::to_string(value) {
                    if let Ok(insider_groups) = serde_json::from_str(string_insider_groups.as_str()) {
                        option.insider_groups = RoleOutlineOptionInsiderGroups::Custom { insider_groups }
                    }
                }
            }
            if let Some(value) = map.get("roleSet") {
                if let Ok(string_role_set) = serde_json::to_string(value) {
                    if let Ok(role_set) = serde_json::from_str(string_role_set.as_str()) {
                        option.roles = RoleOutlineOptionRoles::RoleSet { role_set }
                    }
                }
            } else if let Some(value) = map.get("role") {
                if let Ok(string_role) = serde_json::to_string(value) {
                    if let Ok(role) = serde_json::from_str(string_role.as_str()) {
                        option.roles = RoleOutlineOptionRoles::Role { role }
                    }
                }
            }
        }

        Ok(option)
    }
}
impl RoleOutlineOption {
    /// Makes it so that all roles are specified individually. <br>
    /// Example: if the only town roles enabled are Doctor, Jailor & Auditor and Jailor has been taken then the outline option Town becomes Doctor U Auditor. <br>
    /// Does not include disabled roles or roles whose cap is reached so an empty vector may be returned
    /// The second value is if any roles are role limit 1
    pub fn generator(&self, enabled_roles: &VecSet<Role>) -> VecSet<RoleAssignment> {
        match self.roles.clone() {
            RoleOutlineOptionRoles::Role { role } => {
                if role_can_generate(role, enabled_roles, &[]) {
                    return VecSet::with_first(RoleAssignment{
                        role,
                        insider_groups: self.insider_groups.clone(),
                        win_condition: self.win_condition.clone(),
                    });
                } else {
                    return VecSet::new();
                }
            },
            RoleOutlineOptionRoles::RoleSet {role_set} => {
                let mut options = VecSet::with_capacity(role_set.capacity_hint());                
                for role in role_set.get_roles() {
                    if role_can_generate(role, enabled_roles, &[]){
                        options.insert(
                            RoleAssignment{
                                role,
                                insider_groups: self.insider_groups.clone(),
                                win_condition: self.win_condition.clone(),
                            }
                        );
                    }
                }
                return options;
            },
        }
    }
    /// Returns true if the the win condition & insider groups are the same & the role of lhs are a subset of rhs
    pub fn subset_of(&self, rhs: &RoleOutlineOption) -> bool {
        self.insider_groups == rhs.insider_groups && 
        self.win_condition == rhs.win_condition && 
        self.roles.subset_of(&rhs.roles)
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum RoleOutlineOptionRoles {
    #[serde(rename_all = "camelCase")]
    RoleSet{role_set: RoleSet},
    #[serde(rename_all = "camelCase")]
    Role{role: Role},
}
impl Default for RoleOutlineOptionRoles {
    fn default() -> Self {
        Self::RoleSet { role_set: RoleSet::Any }
    }
}
impl RoleOutlineOptionRoles{
    pub fn get_roles(&self) -> Vec<Role> {
        match self {
            RoleOutlineOptionRoles::RoleSet { role_set } => {
                role_set.get_roles()
            }
            RoleOutlineOptionRoles::Role { role } => 
                vec![*role]
        }
    }
    ///Returns true if this is a subset of rhs
    pub fn subset_of(&self, rhs: &RoleOutlineOptionRoles) -> bool {
        if self == rhs {return true};
        let Self::RoleSet {role_set: rhs} = rhs else {return false};
        if *rhs == RoleSet::Any {return true};

        match self {
            Self::Role{role}=> rhs.contains(role),
            Self::RoleSet{role_set} => role_set.subset_of(*rhs),
        }
    }
}
impl PartialOrd for RoleOutlineOptionRoles {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RoleOutlineOptionRoles {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.get_roles().len().cmp(&self.get_roles().len())
    }
}
impl RoleOutlineOptionRoles {
    pub fn is_role_set(&self) -> bool {
        match self {
            Self::RoleSet{..} => true,
            _=>false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RoleSet {
    Any,

    Town,
    TownCommon,
    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    Mafia,
    MafiaSupport,
    MafiaKilling,

    Cult,
    Fiends,
    
    Neutral,
    Minions
}
impl RoleSet{
    pub fn get_roles(self) -> Vec<Role> {
        match self {
            RoleSet::Any => Role::values(),
            RoleSet::Town => 
                vec![
                    Role::Jailor, Role::Villager, Role::Drunk
                ].into_iter().chain(
                    RoleSet::TownCommon.get_roles()
                ).collect(),
            RoleSet::TownCommon => {
                RoleSet::TownInvestigative.get_roles().into_iter()
                .chain(
                    RoleSet::TownProtective.get_roles()
                ).chain(
                    RoleSet::TownKilling.get_roles()
                ).chain(
                    RoleSet::TownSupport.get_roles()
                ).collect()
            },
            RoleSet::TownInvestigative => 
                vec![
                    Role::Detective, Role::Philosopher, Role::Gossip, 
                    Role::Psychic, Role::Auditor, Role::Spy, 
                    Role::Lookout, Role::Tracker, Role::Snoop,
                    Role::TallyClerk
                ],
            RoleSet::TownProtective => 
                vec![
                    Role::Bodyguard, Role::Cop, Role::Doctor,
                    Role::Bouncer, Role::Engineer, Role::Armorsmith,
                    Role::Steward
                ],
            RoleSet::TownKilling => 
                vec![
                    Role::Vigilante, Role::Veteran, Role::Deputy, Role::Marksman, Role::Rabblerouser
                ],
            RoleSet::TownSupport => 
                vec![
                    Role::Medium, Role::Coxswain,
                    Role::Retributionist, Role::Transporter, Role::Escort, 
                    Role::Mayor, Role::Reporter
                ],
            RoleSet::Mafia =>
                vec![
                    Role::Goon, Role::MafiaSupportWildcard, Role::MafiaKillingWildcard
                ].into_iter().chain(
                    RoleSet::MafiaKilling.get_roles()
                ).chain(
                    RoleSet::MafiaSupport.get_roles()
                ).collect(),
            RoleSet::MafiaKilling => 
                vec![
                    Role::Godfather, Role::Counterfeiter,
                    Role::Impostor, Role::Recruiter,
                    Role::Mafioso, Role::MafiaKillingWildcard
                ],
            RoleSet::MafiaSupport => 
                vec![
                    Role::Blackmailer, Role::Informant, Role::Hypnotist, Role::Consort,
                    Role::Forger, Role::Framer, Role::Mortician, Role::Disguiser,
                    Role::MafiaWitch, Role::Necromancer, Role::Cupid, Role::Reeducator,
                    Role::Ambusher, Role::MafiaSupportWildcard
                ],
            RoleSet::Minions => 
                vec![
                    Role::Witch, Role::Scarecrow, Role::Warper, Role::Kidnapper
                ],
            RoleSet::Neutral =>
                vec![
                    Role::Jester, Role::Revolutionary, Role::Politician, Role::Doomsayer,
                    Role::Martyr, Role::Chronokaiser, Role::SantaClaus, Role::Krampus,
                    Role::Wildcard, Role::TrueWildcard,
                ],
            RoleSet::Fiends =>
                vec![
                    Role::Arsonist, Role::Werewolf, Role::Ojo,
                    Role::Puppeteer, Role::Pyrolisk, Role::Kira,
                    Role::SerialKiller, Role::FiendsWildcard,
                    Role::Spiral, Role::Warden, Role::Yer
                ],
            RoleSet::Cult =>
                vec![
                    Role::Apostle, Role::Disciple, Role::Zealot
                ],
        }
    }
    pub fn contains(self, role: &Role) -> bool {
        return self.get_roles().contains(role);
    }
    /// Returns a number that is an estimate of how many roles are often enabled for a given role set,
    /// or slightly smaller than that if the role set's roles have role limits
    /// For each role set there is comment in the code explaining my thought process
    /// (rl#) mean role limit #
    pub fn capacity_hint(self) -> usize {
        match self {
            // idk what to put here, most games with any are either joke games with everything enabled or 
            // test games where I'm to lazy to select each role, and have like 2 roles enabled.
            RoleSet::Any => 20,
            // I assumed that if you have town instead of town common its because they're the same for you
            // I assumed that the roles that would probably be enabled are:
            //    TI: Auditor, Gossip, Lookout, Philosopher, Psychic, & Snoop or Detective
            //    TP: Armorsmith, Bouncer (rl1), Cop, Doctor, & Engineer
            //    TK: Deputy(rl1), Marksman, Veteran(rl1) & Vigilante
            //    TS: Coxswain or Medium or Retributionist, Transporter, Escort, & Mayor(rl1) or Reporter
            RoleSet::Town | RoleSet::TownCommon => 19,
            // See the Town Common list for reasoning
            RoleSet::TownInvestigative => 6,
            RoleSet::TownProtective => 5,
            RoleSet::TownKilling => 3,
            RoleSet::TownSupport => 4,
            // I assumed that the roles that would probably be enabled is 1 because having it set at mafia means that it could generate SS or SK, 
            // which is incredibly unbalanced so its probably only for testing ig. idk
            RoleSet::Mafia => 1,
            //Godfather, Imposter, Recruiter(assuming my nerf is accepted), or Counterfeiter
            RoleSet::MafiaKilling => 4,
            //Ambusher, Disguiser, Forger, Framer, Hypnotist or Consort, Informant, Mortician
            RoleSet::MafiaSupport => 7,
            //Scarecrow, Warper, Witch
            RoleSet::Minions => 3,
            //Jester & something else
            RoleSet::Neutral => 2,
            //Arsonist, Ojo, Pyrolisk, UZUMAKI, Warden, Werewolf, Yer
            RoleSet::Fiends => 2,
            // Its never really random with cult.
            RoleSet::Cult => 1,
        }
    }
    pub fn subset_of(self, rhs: Self) -> bool {
        if self == RoleSet::Any {return false};
        if self == rhs {return true};
        match rhs {
            Self::Cult | Self::Neutral | 
            Self::MafiaKilling | Self::MafiaSupport |
            Self::Minions | Self::Fiends | 
            Self::TownInvestigative | Self::TownKilling |
            Self::TownProtective | Self::TownSupport => false,

            Self::Any => true,

            Self::TownCommon => self == Self::TownInvestigative || self == Self::TownKilling || self == Self::TownSupport || self == Self::TownProtective,
            
            //intentionally not Self::TownCommon | Self::Town => ... because this also checks for lhs being town common
            Self::Town => self.subset_of(Self::TownCommon),

            Self::Mafia => self == Self::MafiaKilling || self == Self::MafiaSupport,
        }
    }
}



pub fn role_can_generate(role: Role, enabled_roles: &VecSet<Role>, taken_roles: &[Role]) -> bool {
    if !enabled_roles.contains(&role) {
        return false;
    }

    match role.maximum_count() {
        Some(max) => taken_roles.iter().filter(|r|**r==role).count() < max.into(),
        None => true,
    }
}