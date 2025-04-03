use std::collections::HashSet;

use crate::game::{phase::{PhaseState, PhaseType}, player::PlayerReference, Game};

#[derive(Default, Clone)]
pub struct VerdictsToday{
    guilties: HashSet<PlayerReference>,
}

impl Game{
    pub fn verdicts_today(&self)->&VerdictsToday{
        &self.verdicts_today
    }
    pub fn set_verdicts_today(&mut self, verdicts_today: VerdictsToday){
        self.verdicts_today = verdicts_today;
    }
}

impl VerdictsToday{
    pub fn new()->Self{
        Self{
            guilties: HashSet::new(),
        }
    }
    pub fn player_guiltied_today(game: &Game, player: &PlayerReference)->bool{
        game.verdicts_today().guilties.contains(player)
    }
    pub fn guilties(game: &Game)->&HashSet<PlayerReference>{
        &game.verdicts_today().guilties
    }
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        if phase == PhaseType::Obituary {
            game.set_verdicts_today(VerdictsToday::new());
        }
    }
    pub fn before_phase_end(game: &mut Game, _: PhaseType){
        if let PhaseState::Judgement { verdicts, .. } = game.current_phase() {
            let mut verdicts_today = game.verdicts_today().clone();

            verdicts_today.guilties = verdicts.get_guilty_voters().collect();

            game.set_verdicts_today(verdicts_today);
        }
    }
}