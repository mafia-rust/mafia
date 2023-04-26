use serde::{Serialize, Deserialize, de::Visitor};

use self::packet::RoleListEntryPacket;

use super::role::Role;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, PartialEq, Clone, Copy)]
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
            Self::MafiaKilling | Self::MafiaDeception | Self::MafiaSupport 
                => Faction::Mafia,
            Self::TownInvestigative | Self::TownProtective | Self::TownKilling | Self::TownSupport 
                => Faction::Town,
            Self::NeutralEvil | Self::NeutralKilling | Self::NeutralBenign | Self::NeutralChaos 
                => Faction::Neutral,
            Self::CovenEvil 
                => Faction::Coven,
        }
    }
}

pub type RoleList = Vec<RoleListEntry>;

pub fn create_random_roles(role_list: &RoleList) -> Vec<Role> {
    role_list.iter()
        .map(RoleListEntry::get_random_role)
        .collect()
}

pub fn get_all_possible_roles(role_list: &RoleList) -> Vec<Role> {
    //if executioner then add jester
    //if there could be mafioso at beginning then add godfather
    //if any mafia(besides godfather) then add mafioso
    todo!()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoleListEntry {
    Exact {
        faction: Faction,
        faction_alignment: FactionAlignment,
        role: Role,
    },
    FactionAlignment {
        faction: Faction,
        faction_alignment: FactionAlignment,
    },
    Faction {
        faction: Faction,
    },
    Any
}

impl RoleListEntry{
    pub fn get_random_role(&self) -> Role {
        let roles = self.get_possible_roles();

        match roles.get(rand::random::<usize>() % roles.len()) {
            Some(role) => role.clone(),
            None => {
                //if cant find role and was any, crash
                if *self == RoleListEntry::Any{
                    panic!("No roles in get_possible_roles");
                }
                //if cant find role then try again with any
                else{
                    RoleListEntry::Any.get_random_role()
                }
            },
        }
    }
    pub fn get_possible_roles(&self) -> Vec<Role> {
        match self {
            RoleListEntry::Exact {role, .. } => vec![role.clone()],
            RoleListEntry::FactionAlignment { faction_alignment, .. } => 
                Role::values().into_iter().filter(|role|{
                    role.get_faction_alignment() == *faction_alignment
                }).collect(),
            RoleListEntry::Faction { faction, .. } => Role::values().into_iter().filter(|role|{
                role.get_faction_alignment().faction() == *faction
            }).collect(),
            RoleListEntry::Any => Role::values(),
        }
    }
}

impl Serialize for RoleListEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        packet::RoleListEntryPacket::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RoleListEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        RoleListEntryPacket::deserialize(deserializer).map(Into::into)
    }
}

impl From<(Faction, packet::AlignmentPacket)> for FactionAlignment {
    fn from(value: (Faction, packet::AlignmentPacket)) -> Self {
        match value {
            (Faction::Mafia, packet::AlignmentPacket::Killing) => Self::MafiaKilling,
            (Faction::Mafia, packet::AlignmentPacket::Deception) => Self::MafiaDeception,
            (Faction::Mafia, packet::AlignmentPacket::Support) => Self::MafiaSupport,
            (Faction::Town, packet::AlignmentPacket::Killing) => Self::TownKilling,
            (Faction::Town, packet::AlignmentPacket::Support) => Self::TownSupport,
            (Faction::Town, packet::AlignmentPacket::Investigative) => Self::TownInvestigative,
            (Faction::Town, packet::AlignmentPacket::Protective) => Self::TownProtective,
            (Faction::Neutral, packet::AlignmentPacket::Killing) => Self::NeutralKilling,
            (Faction::Neutral, packet::AlignmentPacket::Evil) => Self::NeutralEvil,
            (Faction::Neutral, packet::AlignmentPacket::Benign) => Self::NeutralBenign,
            (Faction::Neutral, packet::AlignmentPacket::Chaos) => Self::NeutralChaos,
            (Faction::Coven, packet::AlignmentPacket::Evil) => Self::CovenEvil,
            (f, a) => panic!("Failed to parse factionalignment {:?} {:?}", f, a)
        }
    }
}

impl From<packet::RoleListEntryPacket> for RoleListEntry {
    fn from(value: packet::RoleListEntryPacket) -> Self {
        match value {
            RoleListEntryPacket::Exact { faction, alignment, role } => {
                RoleListEntry::Exact { faction: faction, faction_alignment: (faction, alignment).into(), role }
            }
            RoleListEntryPacket::Alignment { faction, alignment } => {
                RoleListEntry::FactionAlignment { faction: faction, faction_alignment: (faction, alignment).into() }
            }
            RoleListEntryPacket::Faction { faction } => {
                RoleListEntry::Faction { faction }
            }
            RoleListEntryPacket::Any => {
                RoleListEntry::Any
            }
        }
    }
}

mod packet {
    use crate::game::role::Role;

    use super::{Faction, FactionAlignment, RoleListEntry};

    #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "type")]
    pub(super) enum RoleListEntryPacket {
        Exact {
            faction: Faction,
            alignment: AlignmentPacket,
            role: Role,
        },
        Alignment {
            faction: Faction,
            alignment: AlignmentPacket,
        },
        Faction {
            faction: Faction,
        },
        Any
    }
    
    #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// This is used for serde
    pub(super) enum AlignmentPacket {
        Killing,
        Deception,
        Support,
        Investigative,
        Protective,
        Evil,
        Benign,
        Chaos
    }

    impl From<RoleListEntry> for RoleListEntryPacket {
        fn from(value: RoleListEntry) -> Self {
            match value {
                RoleListEntry::Exact { faction, faction_alignment, role } => {
                    RoleListEntryPacket::Exact { 
                        faction, 
                        alignment: AlignmentPacket::from(faction_alignment),
                        role
                    }
                }
                RoleListEntry::FactionAlignment { faction, faction_alignment } => {
                    RoleListEntryPacket::Alignment { 
                        faction, 
                        alignment: AlignmentPacket::from(faction_alignment) 
                    }
                }
                RoleListEntry::Faction { faction } => {
                    RoleListEntryPacket::Faction { faction }
                }
                RoleListEntry::Any => {
                    RoleListEntryPacket::Any
                }
            }
        }
    }

    impl From<FactionAlignment> for AlignmentPacket {
        fn from(value: FactionAlignment) -> Self {
            match value {
                FactionAlignment::TownInvestigative => Self::Investigative,
                FactionAlignment::CovenEvil 
                | FactionAlignment::NeutralEvil => Self::Evil,
                FactionAlignment::NeutralBenign => Self::Benign,
                FactionAlignment::NeutralChaos => Self::Chaos,
                FactionAlignment::TownKilling 
                | FactionAlignment::MafiaKilling
                | FactionAlignment::NeutralKilling => Self::Killing,
                FactionAlignment::MafiaDeception => Self::Deception,
                FactionAlignment::TownSupport
                | FactionAlignment::MafiaSupport => Self::Support,
                FactionAlignment::TownProtective => Self::Protective
            }
        }
    }
}