use super::player::PlayerIndex;
use super::phase::Phase;

pub struct Grave {
    player: PlayerIndex,
    // shown_role: ShownRole
    shown_will: String,
    died_phase: Phase,
}