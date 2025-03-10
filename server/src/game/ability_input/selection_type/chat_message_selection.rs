use serde::{Deserialize, Serialize};

use crate::game::{
    ability_input::{
        ability_selection::AbilitySelection, AbilityInput, ControllerID, //ValidateAvailableSelection
    }, chat::ChatMessage//, Game
};


#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChatMessageSelection(pub Option<Box<ChatMessage>>);


// #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
// pub struct AvailableChatMessageSelection{}
// impl ValidateAvailableSelection for AvailableChatMessageSelection{
//     type Selection = ChatMessageSelection;
//     fn validate_selection(&self, _game: &Game, _selection: &ChatMessageSelection)->bool{
//         true
//     }
// }


impl AbilityInput{
    pub fn get_chat_message_selection_if_id(&self, id: ControllerID)->Option<ChatMessageSelection>{
        if id != self.id() {return None};
        let AbilitySelection::ChatMessage { selection } = self.selection() else {return None};
        Some(selection)
    }
}