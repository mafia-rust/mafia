use crate::game::{event::on_whisper::{OnWhisper, WhisperFold, WhisperPriority}, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoWhispers;

impl From<&NoWhispers> for ModifierType{
    fn from(_: &NoWhispers) -> Self {
        ModifierType::NoWhispers
    }
}

impl ModifierTrait for NoWhispers {
    fn on_whisper(self, _game: &mut Game, _event: &OnWhisper, fold: &mut WhisperFold, priority: WhisperPriority) {
        if WhisperPriority::Cancel == priority {
            fold.cancelled = true;
            fold.hide_broadcast = true;
        }
    }
}
