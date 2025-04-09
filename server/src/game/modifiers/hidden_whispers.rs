use crate::game::{event::on_whisper::{OnWhisper, WhisperFold, WhisperPriority}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct HiddenWhispers;

/*
    There is modifier specific code in the on_client_message::on_client_message() function
    Specifically in the ToServerPacket::SendWhisper branch of the match statement
*/
impl From<&HiddenWhispers> for ModifierType{
    fn from(_: &HiddenWhispers) -> Self {
        ModifierType::HiddenWhispers
    }
}

impl ModifierTrait for HiddenWhispers {
    fn on_whisper(self, _game: &mut Game, _event: &OnWhisper, fold: &mut WhisperFold, priority: WhisperPriority) {
        if WhisperPriority::Cancel == priority {
            fold.hide_broadcast = true;
        }
    }
}