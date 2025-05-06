use serde::{Deserialize, Serialize};

use crate::game::{
    ability_input::{
        ability_selection::AbilitySelection, AbilityInput, AvailableSelectionKind, ControllerID //ValidateAvailableSelection
    }, chat::ChatMessage//, Game
};


#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChatMessageSelection(pub Option<Box<ChatMessage>>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AvailableChatMessageSelection;
impl AvailableSelectionKind for AvailableChatMessageSelection{
    type Selection = ChatMessageSelection;

    fn validate_selection(&self, _game: &crate::game::Game, _selection: &Self::Selection)->bool {
        true
    }

    fn default_selection(&self, _game: &crate::game::Game) -> Self::Selection {
        ChatMessageSelection(None)
    }
}


impl AbilityInput{
    pub fn get_chat_message_selection_if_id(&self, id: ControllerID)->Option<ChatMessageSelection>{
        if id != self.id() {return None};
        let AbilitySelection::ChatMessage(selection) = self.selection() else {return None};
        Some(selection)
    }
}