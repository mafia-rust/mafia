use std::time::Duration;

use super::{settings::PhaseTimeSettings, Game};


#[derive(Clone, Copy, PartialEq)]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    Evening,
    Night,
}

pub struct PhaseStateMachine {
    pub time_remaining: Duration,
    pub current_state: PhaseType,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl PhaseStateMachine {
    pub fn new(times: PhaseTimeSettings) -> Self {
        let current_state = PhaseType::Morning;

        Self {
            time_remaining: current_state.get_length(&times),
            day_number: 1,
            current_state,
        }
    }
    pub fn tick(&mut self, game: &Game, time_passed: Duration){
        self.time_remaining -= time_passed;
        
        if self.time_remaining > Duration::ZERO{
            return;
        }

        //call end
        self.current_state = self.current_state.end();
        //fix time
        self.time_remaining += self.current_state.get_length(&game.settings.phase_times);
        //call start
        self.current_state.start();
    }
}

impl PhaseType {
    pub const fn get_length(&self, times: &PhaseTimeSettings) -> Duration {
        match self {
            PhaseType::Morning => times.morning,
            PhaseType::Discussion => times.discussion,
            PhaseType::Voting => times.voting,
            PhaseType::Testimony => times.testimony,
            PhaseType::Judgement => times.judgement,
            PhaseType::Evening => times.evening,
            PhaseType::Night => times.night,
        }
    }

    pub fn start(&self) {
        // Match phase type and do stuff
        match self {
            PhaseType::Morning => todo!(),
            PhaseType::Discussion => todo!(),
            PhaseType::Voting => todo!(),
            PhaseType::Testimony => todo!(),
            PhaseType::Judgement => todo!(),
            PhaseType::Evening => todo!(),
            PhaseType::Night => todo!(),
        }
    }

    ///returns the next phase
    pub fn end(&self) -> PhaseType {
        // Match phase type and do stuff
        match self {
            PhaseType::Morning => todo!(),
            PhaseType::Discussion => todo!(),
            PhaseType::Voting => todo!(),
            PhaseType::Testimony => todo!(),
            PhaseType::Judgement => todo!(),
            PhaseType::Evening => todo!(),
            PhaseType::Night => todo!(),
        }
    }

    pub fn is_day(&self) -> bool {
        matches!(self, PhaseType::Night)
    }

}