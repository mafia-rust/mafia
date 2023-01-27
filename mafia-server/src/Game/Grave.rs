mod Grave;

pub struct Grave{
    player: Player,
    ////shown_role  : String?enum ShownRole?
    shown_will : String,

    //these next 2 might want to be both combined into a single struct
    died_phase : PhaseState, //should this be a reference? A string? PhaseStateID?
    died_phase_number : u32,
}