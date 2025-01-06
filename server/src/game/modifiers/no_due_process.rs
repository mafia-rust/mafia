use crate::game::{phase::{PhaseState, PhaseStateMachine}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct AutoGuilty;

impl From<&AutoGuilty> for ModifierType{
    fn from(_: &AutoGuilty) -> Self {
        ModifierType::AutoGuilty
    }
}

impl ModifierTrait for AutoGuilty{
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        match phase {
            PhaseState::Testimony { player_on_trial, .. }
            | PhaseState::Judgement { player_on_trial, .. } => {
                PhaseStateMachine::next_phase(game, Some(PhaseState::FinalWords { player_on_trial }))
            }
            _ => {}
        }
    }
}
