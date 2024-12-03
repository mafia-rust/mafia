pub mod selection_type; pub use selection_type::*;
pub mod ability_selection; pub use ability_selection::*;
pub mod saved_controllers_map; pub use saved_controllers_map::*;
pub mod controller_id; pub use controller_id::*;
pub mod available_ability; pub use available_ability::*;

use serde::{Deserialize, Serialize};

use super::{
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference, Game
};






#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AbilityInput{
    id: ControllerID, 
    selection: AbilitySelection
}
impl AbilityInput{
    pub fn new(id: ControllerID, selection: AbilitySelection)->Self{
        Self{id, selection}
    }
    pub fn id(&self)->ControllerID{
        self.id.clone()
    }
    pub fn selection(&self)->AbilitySelection{
        self.selection.clone()
    }
}





pub trait ValidateAvailableSelection{
    type Selection;
    fn validate_selection(&self, game: &Game, selection: &Self::Selection)->bool;
}

impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        OnAbilityInputReceived::new(actor_ref, self.clone()).invoke(game);
    }
}