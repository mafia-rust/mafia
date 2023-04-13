use super::player::PlayerIndex;

//#[derive(Clone)]
pub struct Visit {
    pub target: PlayerIndex,

    pub astral: bool,
    pub attack: bool,
}