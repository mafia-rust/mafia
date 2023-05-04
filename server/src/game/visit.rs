use super::player::PlayerReference;

//#[derive(Clone)]
pub struct Visit {
    pub target: PlayerReference,

    pub astral: bool,
    pub attack: bool,
}