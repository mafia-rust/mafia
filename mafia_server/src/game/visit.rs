use super::player::PlayerID;

pub struct Visit {
    target: PlayerID,

    astral: bool,
    attack: bool,
}