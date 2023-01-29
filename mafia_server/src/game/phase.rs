// This is a placeholder for now
pub type PhaseTime = std::time::Duration;

#[derive(Clone, Copy)]
pub enum PhaseType {
    Morning,
    Discussion, 
    Voting, 
    Testimony, 
    Judgement, 
    FinalWords, 
    Night
}

#[derive(Clone, Copy)]
pub struct Phase {
    phase_type: PhaseType,
    number: u8, // Hopefully nobody is having more than 256 days anyway
}

pub struct PhaseStateMachine {
    pub time_remaining: PhaseTime,
    pub current_state: Phase,
}

impl PhaseStateMachine {
    pub const fn new() -> Self {
        Self {
            time_remaining: PhaseTime::from_secs(0),
            current_state: Phase {
                phase_type: PhaseType::Morning,
                number: 0,
            },
        }
    }
}

impl PhaseType {
    pub fn get_length(&self) {
        todo!();    // Sammy knows the time lengths I'm sure
    }

    pub fn start(&self) {
        // Match phase type and do stuff
    }

    pub fn end(&self) {
        // Match phase type and do stuff
    }
}