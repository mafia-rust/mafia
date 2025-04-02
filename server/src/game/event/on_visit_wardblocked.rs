use crate::game::{ 
    chat::ChatMessageVariant, components::{mafia::Mafia, syndicate_gun_item::SyndicateGunItem}, player::PlayerReference, visit::Visit, Game
};

use super::on_midnight::MidnightVariables;

#[must_use = "Event must be invoked"]
pub struct OnVisitWardblocked{
    visit: Visit
}
impl OnVisitWardblocked{
    pub fn new(visit: Visit) -> Self{
        Self{visit}
    }
    pub fn invoke(self, game: &mut Game, midnight_variables: &mut MidnightVariables){
        self.visit.visitor.set_night_blocked(midnight_variables, true);
        self.visit.visitor.push_night_message(midnight_variables, ChatMessageVariant::Wardblocked);

        for player_ref in PlayerReference::all_players(game){
            player_ref.on_visit_wardblocked(game, self.visit);
        }
        Mafia::on_visit_wardblocked(game, self.visit);
        SyndicateGunItem::on_visit_wardblocked(game, self.visit);
    }
}