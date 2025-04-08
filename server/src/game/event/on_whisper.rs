use crate::{game::{modifiers::Modifiers, player::PlayerReference, Game}, event_priority};
use super::Event;

#[derive(Clone)]
pub struct OnWhisper {
    pub sender: PlayerReference,
    pub receiver: PlayerReference,
    pub message: String,
}
pub struct WhisperFold {
    pub cancelled: bool,
    pub hide_broadcast: bool
}
event_priority!(WhisperPriority{Cancel, Broadcast, Send});

impl OnWhisper {
    pub fn new(sender: PlayerReference, receiver: PlayerReference, message: String) -> Self {
        Self {
            sender,
            receiver,
            message,
        }
    }
}

impl Event for OnWhisper {
    type FoldValue = WhisperFold;
    type Priority = WhisperPriority;

    fn listeners() -> Vec<super::EventListenerFunction<Self>> {
        vec![
            Game::on_whisper,
            Modifiers::on_whisper,
            PlayerReference::on_whisper,
        ]
    }

    fn initial_fold_value(&self) -> Self::FoldValue {
        WhisperFold {
            cancelled: false,
            hide_broadcast: false,
        }
    }
}