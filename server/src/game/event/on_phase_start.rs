use crate::game::{
    ability_input::saved_controllers_map::SavedControllersMap, components::{
        confused::Confused, cult::Cult, detained::Detained, drunk_aura::DrunkAura, mafia::Mafia, night_visits::NightVisits, verdicts_today::VerdictsToday
    }, modifiers::Modifiers, phase::PhaseState, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnPhaseStart{
    phase: PhaseState
}
impl OnPhaseStart{
    pub fn new(phase: PhaseState) -> Self{
        Self{ phase }
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_phase_start(game, self.phase.phase());
        }

        NightVisits::on_phase_start(game, self.phase.phase());
        Detained::on_phase_start(game, self.phase.phase());
        VerdictsToday::on_phase_start(game, self.phase.phase());
        Mafia::on_phase_start(game, self.phase.phase());
        Cult::on_phase_start(game, self.phase.phase());
        SavedControllersMap::on_phase_start(game, self.phase.phase());
        Modifiers::on_phase_start(game, self.phase.clone());
        Confused::on_phase_start(game, self.phase.clone());
        DrunkAura::on_phase_start(game, self.phase.clone());

        game.on_phase_start(self.phase.phase());
    }
}