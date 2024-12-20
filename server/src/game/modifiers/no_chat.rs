use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoChat;

impl From<&NoChat> for ModifierType{
    fn from(_: &NoChat) -> Self {
        ModifierType::NoChat
    }
}

impl ModifierTrait for NoChat {}
