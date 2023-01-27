use super::player::Player;
use super::phase::Phase;

pub struct Grave {
    player: Player,
    ////shown_role  : String?enum ShownRole?
    shown_will : String,

    died_phase : Phase,
}