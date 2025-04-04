use serde::{Deserialize, Serialize};

use crate::game::{
    ability_input::{
        ability_selection::AbilitySelection, AbilityInput, ControllerID, AvailableSelectionKind
    },
    Game
};


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerSelection(pub i8);


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AvailableIntegerSelection {
    Bounded {
        min: i8,    //inclusive
        max: i8
    },
    Discrete {
        values: Vec<i8>
    }
}

impl Default for AvailableIntegerSelection {
    fn default() -> Self {
        Self::Bounded { min: 0, max: 0 }
    }
}

impl AvailableSelectionKind for AvailableIntegerSelection{
    type Selection = IntegerSelection;
    fn validate_selection(&self, _game: &Game, selection: &IntegerSelection)->bool{
        match self {
            Self::Bounded { min, max } => selection.0 >= *min && selection.0 <= *max,
            Self::Discrete { values } => values.contains(&selection.0),
        }
        
    }
    
    fn default_selection(&self, _: &Game) -> Self::Selection {
        IntegerSelection(0)
    }
}


impl AbilityInput{
    pub fn get_integer_selection_if_id(&self, id: ControllerID)->Option<IntegerSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Integer(selection) = self.selection() else {return None};
        Some(selection)
    }
}