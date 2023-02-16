// This is a placeholder for now
pub type PhaseTime = std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    FinalWords,
    Night,
}

impl PhaseType {
    pub fn is_day(&self) -> bool {
        matches!(self, PhaseType::Night)
    }
}

#[derive(Clone, Copy)]
pub struct Phase {
    pub phase_type: PhaseType,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
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
        let phase_type = PhaseType::Morning;
        Self {
            time_remaining: phase_type.get_length(),
            current_state: Phase {
                phase_type,
                day_number: 0,
            },
        }
    }
}

impl PhaseType {
    pub const fn get_length(&self) -> PhaseTime {
        todo!();    // Sammy knows the time lengths I'm sure
    }

    pub fn start(&self) {
        // Match phase type and do stuff
    }

    pub fn end(&self) {
        // Match phase type and do stuff
    }
}