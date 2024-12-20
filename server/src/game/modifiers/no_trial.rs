use crate::game::{phase::{PhaseState, PhaseStateMachine}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoTrial;

impl From<&NoTrial> for ModifierType{
    fn from(_: &NoTrial) -> Self {
        ModifierType::NoTrial
    }
}

impl ModifierTrait for NoTrial{
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
