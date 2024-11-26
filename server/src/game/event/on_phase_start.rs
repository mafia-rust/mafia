use crate::game::{
    components::{
        cult::Cult, detained::Detained, generic_ability::GenericAbilitySaveComponent, mafia::Mafia, night_visits::NightVisits, pitchfork::Pitchfork, syndicate_gun_item::SyndicateGunItem, verdicts_today::VerdictsToday
    },
    phase::PhaseType, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnPhaseStart{
    phase: PhaseType
}
impl OnPhaseStart{
    pub fn new(phase: PhaseType) -> Self{
        Self{ phase }
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_phase_start(game, self.phase);
        }

        NightVisits::on_phase_start(game, self.phase);
        Detained::on_phase_start(game, self.phase);
        VerdictsToday::on_phase_start(game, self.phase);
        Mafia::on_phase_start(game, self.phase);
        Cult::on_phase_start(game, self.phase);
        Pitchfork::on_phase_start(game, self.phase);
        GenericAbilitySaveComponent::on_phase_start(game, self.phase);
        SyndicateGunItem::on_phase_start(game, self.phase);

        game.on_phase_start(self.phase);
    }
}