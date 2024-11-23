use super::player::PlayerReference;

#[derive(Copy, Clone, Debug)]
pub struct Visit {
    pub visitor: PlayerReference,
    pub target: PlayerReference,

    pub tag: VisitTag,
    pub attack: bool,
}
impl Visit {
    pub fn new(visitor: PlayerReference, target: PlayerReference, attack: bool) -> Self {
        Self {
            visitor,
            target,
            attack,
            tag: VisitTag::None,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisitTag{
    #[default]
    None,
    SyndicateGunItem,
}