use crate::game::{
    phase::PhaseType, player::PlayerReference, visit::Visit, Game
};

#[derive(Default)]
pub struct NightVisits{
    pub visits: Vec<Visit>,
}


impl NightVisits{
    //event listeners
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        if phase == PhaseType::Night{
            Self::clear_all_visits(game);
        }
    }




    // mutators
    fn clear_all_visits(game: &mut Game){
        game.night_visits.visits.clear();
    }

    fn clear_visits_with_predicate(game: &mut Game, predicate: impl Fn(&Visit) -> bool){
        game.night_visits.visits.retain(|visit| !predicate(visit));
    }

    fn clear_visits_from_visitor(game: &mut Game, visitor: PlayerReference){
        Self::clear_visits_with_predicate(game, |visit| visit.visitor == visitor);
    }
    fn add_visits(game: &mut Game, visits: Vec<Visit>){
        game.night_visits.visits.extend(visits);
    }

    //accessors
    fn get_visits_from_visitor<'a>(game: &Game, visitor: PlayerReference) -> Vec<&Visit>{
        game.night_visits.visits.iter().filter(|visit| visit.visitor == visitor).collect()
    }
}

impl PlayerReference{
    pub fn night_visits<'a>(&self, game: &'a Game) -> Vec<&'a Visit>{
        NightVisits::get_visits_from_visitor(game, *self)
    }
    pub fn night_visits_cloned<'a>(&self, game: &Game) -> Vec<Visit>{
        NightVisits::get_visits_from_visitor(game, *self)
            .into_iter()
            .cloned()
            .collect()
    }
    pub fn set_night_visits(&self, game: &mut Game, visits: Vec<Visit>){
        NightVisits::clear_visits_from_visitor(game, *self);
        NightVisits::add_visits(game, visits);
    }
}