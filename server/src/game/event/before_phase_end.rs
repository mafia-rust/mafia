use crate::game::{
    components::{confused::Confused, drunk_aura::DrunkAura, pitchfork::Pitchfork, verdicts_today::VerdictsToday},
    modifiers::Modifiers, phase::PhaseType, Game
};

#[must_use = "Event must be invoked"]
pub struct BeforePhaseEnd{
    phase: PhaseType
}
impl BeforePhaseEnd{
    pub fn new(phase: PhaseType) -> Self{
        Self{ phase }
    }
    pub fn invoke(self, game: &mut Game){
        VerdictsToday::before_phase_end(game, self.phase);
        Pitchfork::before_phase_end(game, self.phase);
        Modifiers::before_phase_end(game, self.phase);
        Confused::before_phase_end(game, self.phase);
        DrunkAura::before_phase_end(game, self.phase);
    }
}