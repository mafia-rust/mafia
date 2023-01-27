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

pub struct PhaseStateMachine {
    number: u8, // Hopefully nobody is having more than 256 days anyway
    time_remaining: PhaseTime,
    current_state: PhaseType,
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