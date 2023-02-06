use super::player::PlayerID;
use super::phase::Phase;

pub struct Grave {
    player: PlayerID,
    // shown_role: ShownRole
    shown_will: String,
    died_phase: Phase,
}