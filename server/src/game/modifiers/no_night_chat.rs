use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoNightChat;

impl From<&NoNightChat> for ModifierType{
    fn from(_: &NoNightChat) -> Self {
        ModifierType::NoNightChat
    }
}

impl ModifierTrait for NoNightChat {}
