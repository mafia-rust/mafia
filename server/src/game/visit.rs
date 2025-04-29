use super::{player::PlayerReference, role::Role};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Visit {
    pub visitor: PlayerReference,
    pub target: PlayerReference,

    pub tag: VisitTag,
    pub attack: bool,
}
impl Visit {
    pub fn new_role(visitor: PlayerReference, target: PlayerReference, attack: bool, role: Role, id: u8) -> Self {
        Self {
            visitor,
            target,
            attack,
            tag: VisitTag::Role{role, id},
        }
    }
    pub fn new(visitor: PlayerReference, target: PlayerReference, attack: bool, tag: VisitTag) -> Self {
        Self {
            visitor,
            target,
            attack,
            tag,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisitTag{
    Role{role: Role, id: u8},
    SyndicateGunItem,
    SyndicateBackupAttack
}