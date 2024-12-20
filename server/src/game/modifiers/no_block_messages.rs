use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoBlockMessages;

impl From<&NoBlockMessages> for ModifierType{
    fn from(_: &NoBlockMessages) -> Self {
        ModifierType::NoBlockMessages
    }
}

impl ModifierTrait for NoBlockMessages {}
