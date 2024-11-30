use serde::{Deserialize, Serialize};

use crate::game::role::Role;

pub type RoleAbilityID = u8;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AbilityID{
    #[serde(rename_all = "camelCase")]
    Role{role: Role, id: RoleAbilityID},
    ForfeitVote,
    PitchforkVote,
    SyndicateGunItemShoot,
    SyndicateGunItemGive,
}
impl AbilityID{
    pub fn role(role: Role, role_ability_id: RoleAbilityID)->Self{
        Self::Role{role, id: role_ability_id}
    }
    pub fn forfeit_vote()->Self{
        Self::ForfeitVote
    }
    pub fn pitchfork_vote()->Self{
        Self::PitchforkVote
    }
    pub fn syndicate_gun_item_shoot()->Self{
        Self::SyndicateGunItemShoot
    }
    pub fn syndicate_gun_item_give()->Self{
        Self::SyndicateGunItemGive
    }
}