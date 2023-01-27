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

pub struct Phase {
    phase_type : PhaseType,
    number: u8, // Hopefully nobody is having more than 256 days anyway
    length : PhaseTime,
}

impl Phase {
    pub fn start(&self) {
        // Match phase type and do stuff
    }

    pub fn end(&self) {
        // Match phase type and do stuff
    }
}