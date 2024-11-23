use super::player::PlayerReference;

#[derive(Copy, Clone, Debug)]
pub struct Visit {
    pub visitor: PlayerReference,
    pub target: PlayerReference,

    pub attack: bool,
}
impl Visit {
    pub fn new(visitor: PlayerReference, target: PlayerReference, attack: bool) -> Self {
        Self {
            visitor,
            target,
            attack,
        }
    }
}