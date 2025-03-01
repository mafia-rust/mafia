use crate::game::{phase::{PhaseState, PhaseType}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct SkipDay1;

impl From<&SkipDay1> for ModifierType{
    fn from(_: &SkipDay1) -> Self {
        ModifierType::SkipDay1
    }
}

impl ModifierTrait for SkipDay1{
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        if game.day_number() != 1 {return;}
        if phase.phase() == PhaseType::Briefing {return;}
        game.on_fast_forward();
    }
}
