use super::player::PlayerReference;

#[derive(Clone)]
pub struct Visit {
    pub target: PlayerReference,

    pub attack: bool,
}
impl Visit {
    pub fn new(target: PlayerReference, attack: bool) -> Self {
        Self {
            target,
            attack,
        }
    }
}