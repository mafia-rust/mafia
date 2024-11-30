pub mod selection_type;
pub mod ability_selection;
pub mod saved_ability_inputs;
pub mod ability_id;
pub mod available_abilities_data;

use ability_id::AbilityID;
use ability_selection::AbilitySelection;

use serde::{Deserialize, Serialize};

use super::{
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference, Game
};






#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AbilityInput{
    id: AbilityID, 
    selection: AbilitySelection
}
impl AbilityInput{
    pub fn new(id: AbilityID, selection: AbilitySelection)->Self{
        Self{id, selection}
    }
    pub fn id(&self)->AbilityID{
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