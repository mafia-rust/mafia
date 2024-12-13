use crate::game::{
    ability_input::ControllerID, components::mafia::Mafia, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnControllerSelectionChanged{
    id: ControllerID,
}
impl OnControllerSelectionChanged{
    pub fn new(id: ControllerID) -> Self{
        Self{id}
    }
    pub fn invoke(self, game: &mut Game){
        Mafia::on_controller_selection_changed(game, self.id.clone());
        for player in PlayerReference::all_players(game){
            player.on_controller_selection_changed(game, self.id.clone());
        }
    }
}