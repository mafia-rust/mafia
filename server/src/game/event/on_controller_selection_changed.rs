use crate::game::{
    ability_input::ControllerID, components::mafia::Mafia, Game
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
        Mafia::on_controller_selection_changed(game, self.id);
    }
}