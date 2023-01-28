use super::player::PlayerID;
use super::phase::Phase;

pub struct Grave {
    player: PlayerID,
    ////shown_role  : String?enum ShownRole?
    shown_will : String,

    died_phase : Phase,
}