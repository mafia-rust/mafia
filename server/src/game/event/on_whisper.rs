use crate::game::{modifiers::Modifiers, player::PlayerReference, Game};

use super::{Event, EventPriority};



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WhisperPriority {
    Cancel,
    Broadcast,
    Send
}

impl EventPriority for WhisperPriority {
    fn first() -> Self {
        WhisperPriority::Cancel
    }

    fn next(self) -> Option<Self> {
        match self {
            WhisperPriority::Cancel => Some(WhisperPriority::Broadcast),
            WhisperPriority::Broadcast => Some(WhisperPriority::Send),
            WhisperPriority::Send => None
        }
    }
}

pub struct WhisperFold {
    pub cancelled: bool,
    pub hide_broadcast: bool
}


#[derive(Clone)]
pub struct OnWhisper {
    pub sender: PlayerReference,
    pub receiver: PlayerReference,
    pub message: String,
}

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
    type Inner = Self;
    type Priority = WhisperPriority;

    fn listeners() -> Vec<super::EventListenerFunction<Self>> {
        vec![
            Game::on_whisper,
            Modifiers::on_whisper
        ]
    }

    fn initial_fold_value(&self) -> Self::FoldValue {
        WhisperFold {
            cancelled: false,
            hide_broadcast: false,
        }
    }

    fn inner(&self) -> Self::Inner {
        self.clone()
    }
}