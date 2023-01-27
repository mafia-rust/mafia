// This is a placeholder for now
pub type PhaseTime = std::time::Duration;

pub enum PhaseType {
    Morning,
    Discussion, 
    Voting, 
    Testimony, 
    Judgement, 
    FinalWords, 
    Night
}

pub struct PhaseState {
    phase_type : PhaseType,
    time_length : PhaseTime,
}

pub struct PhaseStateMachine {
    number: u8, // Hopefully nobody is having more than 256 days anyway
    time_remaining: PhaseTime,
    current_state: PhaseState,
}

impl PhaseType {
    pub fn start(&self) {
        // Match phase type and do stuff
    }

    pub fn end(&self) {
        // Match phase type and do stuff
    }
}