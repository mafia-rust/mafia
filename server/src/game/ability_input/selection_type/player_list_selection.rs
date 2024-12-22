use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::{ability_selection::AbilitySelection, ControllerID, AbilityInput, ValidateAvailableSelection}, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerListSelection(pub Vec<PlayerReference>);


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePlayerListSelection{
    pub available_players: VecSet<PlayerReference>,
    pub can_choose_duplicates: bool,
    pub max_players: Option<u8>
}
impl ValidateAvailableSelection for AvailablePlayerListSelection{
    type Selection = PlayerListSelection;
    fn validate_selection(&self, _game: &Game, selection: &PlayerListSelection)->bool{
        self.available_players.is_superset(&selection.0.iter().cloned().collect()) && 
        (self.can_choose_duplicates || selection.0.len() == selection.0.iter().collect::<Vec<_>>().len()) &&
        self.max_players.map_or(true, |max| selection.0.len() <= max as usize)
    }
}


impl AbilityInput{
    pub fn get_player_list_selection_if_id(&self, id: ControllerID)->Option<PlayerListSelection>{
        if id != self.id() {return None};
        let AbilitySelection::PlayerList { selection } = self.selection() else {return None};
        Some(selection)
    }
}