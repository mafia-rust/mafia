use serde::{Deserialize, Serialize};

use crate::game::{
    ability_input::{
        ability_selection::AbilitySelection, ControllerID,
        AbilityInput, ValidateAvailableSelection
    },
    Game
};


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerSelection(pub i8);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AvailableIntegerSelection{
    pub min: i8,    //inclusive
    pub max: i8
}
impl ValidateAvailableSelection for AvailableIntegerSelection{
    type Selection = IntegerSelection;
    fn validate_selection(&self, _game: &Game, selection: &IntegerSelection)->bool{
        selection.0 >= self.min && selection.0 <= self.max
    }
}


impl AbilityInput{
    pub fn get_integer_selection_if_id(&self, id: ControllerID)->Option<IntegerSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Integer { selection } = self.selection() else {return None};
        Some(selection)
    }
}