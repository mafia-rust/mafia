use crate::game::{
    phase::PhaseType, player::PlayerReference, visit::{Visit, VisitTag}, Game
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

    pub fn clear_visits_with_predicate(game: &mut Game, predicate: impl Fn(&Visit) -> bool){
        game.night_visits.visits.retain(|visit| !predicate(visit));
    }

    fn clear_visits_from_visitor(game: &mut Game, visitor: PlayerReference){
        Self::clear_visits_with_predicate(game, |visit| visit.visitor == visitor);
    }
    pub fn add_visit(game: &mut Game, visits: Visit){
        game.night_visits.visits.push(visits);
    }
    fn add_visits(game: &mut Game, visits: Vec<Visit>){
        game.night_visits.visits.extend(visits);
    }
    pub fn all_visits(game: &Game) -> Vec<&Visit>{
        game.night_visits.visits.iter().collect()
    }

    //accessors
    fn get_untagged_visits_from_visitor<'a>(game: &Game, visitor: PlayerReference) -> Vec<&Visit>{
        game.night_visits.visits.iter()
            .filter(|visit| visit.visitor == visitor)
            .filter(|visit| visit.tag == VisitTag::Role)
            .collect()
    }
    fn get_untagged_visits_from_visitor_mut<'a>(game: &'a mut Game, visitor: PlayerReference) -> Vec<&'a mut Visit>{
        game.night_visits.visits.iter_mut()
            .filter(|visit| visit.visitor == visitor)
            .filter(|visit| visit.tag == VisitTag::Role)
            .collect()
    }
}

impl PlayerReference{
    pub fn untagged_night_visits<'a>(&self, game: &'a Game) -> Vec<&'a Visit>{
        NightVisits::get_untagged_visits_from_visitor(game, *self)
    }
    pub fn untagged_night_visits_mut<'a>(&self, game: &'a mut Game) -> Vec<&'a mut Visit>{
        NightVisits::get_untagged_visits_from_visitor_mut(game, *self)
    }
    pub fn untagged_night_visits_cloned<'a>(&self, game: &Game) -> Vec<Visit>{
        NightVisits::get_untagged_visits_from_visitor(game, *self)
            .into_iter()
            .cloned()
            .collect()
    }
    /// Returns all vists where the player is the visitor
    pub fn all_night_visits_cloned<'a>(&self, game: &Game) -> Vec<Visit>{
        NightVisits::all_visits(game)
            .into_iter()
            .filter(|visit| visit.visitor == *self)
            .cloned()
            .collect()
    }
    /// Returns all vists where the player is the target
    pub fn all_night_visitors_cloned(self, game: &Game) -> Vec<PlayerReference> {
        NightVisits::all_visits(game)
            .into_iter()
            .filter(|visit| visit.target == self)
            .map(|visit| visit.visitor)
            .collect()
    }
    pub fn set_night_visits(&self, game: &mut Game, visits: Vec<Visit>){
        NightVisits::clear_visits_from_visitor(game, *self);
        NightVisits::add_visits(game, visits);
    }
}