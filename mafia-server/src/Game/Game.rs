
pub(crate) struct Game{
    players: HashMap<PlayerID, Player>,
    graves: Vec<Grave>,

    //RoleList
    //PhaseTimes
    //Investigator Results
    //other settings

    //these next 2 might want to be both combined into a single struct
    current_phase : PhaseState, 
    current_phase_number : u32,
}