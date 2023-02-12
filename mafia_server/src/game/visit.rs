use super::player::PlayerIndex;

pub struct Visit {
    target: PlayerIndex,

    astral: bool,
    attack: bool,
}