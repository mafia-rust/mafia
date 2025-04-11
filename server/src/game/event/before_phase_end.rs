use crate::game::{
    components::{pitchfork::Pitchfork, verdicts_today::VerdictsToday}, modifiers::Modifiers, phase::PhaseType, player::PlayerReference, Game
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
        for player_ref in PlayerReference::all_players(game) {
            player_ref.before_phase_end(game, self.phase);
        }
        
        VerdictsToday::before_phase_end(game, self.phase);
        Pitchfork::before_phase_end(game, self.phase);
        Modifiers::before_phase_end(game, self.phase);
    }
}