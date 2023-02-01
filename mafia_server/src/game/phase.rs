// This is a placeholder for now
pub type PhaseTime = std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    FinalWords,
    Night,
}

impl From<PhaseType> for u8 {
    fn from(phase_type: PhaseType) -> Self {
        phase_type as u8
    }
}

impl PhaseType {
    pub fn is_day(&self) -> bool {
        match self {
            PhaseType::Morning | PhaseType::Discussion | 
            PhaseType::Voting | PhaseType::Testimony | 
            PhaseType::Judgement | PhaseType::FinalWords => true,
            PhaseType::Night => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Phase {
    pub phase_type: PhaseType,
    pub number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl Phase {
    pub fn is_day(&self) -> bool {
        self.phase_type.is_day()
    }
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