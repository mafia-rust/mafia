use serde::{Deserialize, Serialize};

use crate::{
    game::{
        ability_input::{
            ability_selection::AbilitySelection, ControllerID,
            AbilityInput, ValidateAvailableSelection
        },
        player::PlayerReference, role::kira::KiraGuess, Game
    },
    vec_map::VecMap,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct KiraSelection(pub VecMap<PlayerReference, KiraGuess>);
impl KiraSelection{
    pub fn new(map: VecMap<PlayerReference, KiraGuess>) -> Self{
        KiraSelection(map)
    }
}

impl PartialOrd for KiraSelection{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for KiraSelection{
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableKiraSelection{
    count_must_guess: u8
}
impl ValidateAvailableSelection for AvailableKiraSelection{
    type Selection = KiraSelection;
    fn validate_selection(&self, game: &Game, selection: &KiraSelection)->bool{
        let count: usize = self.count_must_guess.into();
        selection.0.len() == count &&
        selection.0.iter().all(|p|p.0.alive(game))
    }
}

impl AvailableKiraSelection{
    pub fn new(count_must_guess: u8)->Self{
        AvailableKiraSelection { count_must_guess }
    }
}


impl AbilityInput{
    pub fn get_kira_selection_if_id(&self, id: ControllerID)->Option<KiraSelection>{
        if id != self.id() {return None};
        let AbilitySelection::Kira { selection } = self.selection() else {return None};
        Some(selection)
    }
}