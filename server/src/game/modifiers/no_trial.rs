use crate::game::{phase::{PhaseState, PhaseStateMachine}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoTrialPhases;

impl From<&NoTrialPhases> for ModifierType{
    fn from(_: &NoTrialPhases) -> Self {
        ModifierType::NoTrialPhases
    }
}

impl ModifierTrait for NoTrialPhases{
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        match phase {
            PhaseState::Nomination { .. }
            | PhaseState::Testimony { .. }
            | PhaseState::Judgement { .. } => {
                PhaseStateMachine::next_phase(game, Some(PhaseState::Dusk))
            }
            _ => {}
        }
    }
}
