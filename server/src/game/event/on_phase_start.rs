use crate::game::{phase::PhaseType, player::PlayerReference, Game};

#[must_use = "Event must be invoked"]
pub struct OnPhaseStart{
    pub phase: PhaseType
}
impl OnPhaseStart{
    pub fn new(phase: PhaseType) -> Self{
        Self{ phase }
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_phase_start(game, self.phase);
        }

        game.mafia().on_phase_start(game, self.phase);
        game.cult().on_phase_start(game, self.phase);

        game.on_phase_start(self.phase);
    }
    pub fn create_and_invoke(game: &mut Game, phase: PhaseType){
        Self::new(phase).invoke(game);
    }
}