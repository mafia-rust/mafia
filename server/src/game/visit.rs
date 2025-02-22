use serde::Serialize;

use super::player::PlayerReference;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, PartialOrd, Ord)]
pub struct Visit {
    pub visitor: PlayerReference,
    pub target: PlayerReference,

    pub tag: VisitTag,
    pub attack: bool,
}
impl Visit {
    pub fn new_none(visitor: PlayerReference, target: PlayerReference, attack: bool) -> Self {
        Self {
            visitor,
            target,
            attack,
            tag: VisitTag::Role,
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, PartialOrd, Ord)]
pub enum VisitTag{
    #[default]
    Role,
    SyndicateGunItem,
    SyndicateBackupAttack
}