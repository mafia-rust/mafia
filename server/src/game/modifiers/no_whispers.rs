use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoWhispers;

impl From<&NoWhispers> for ModifierType{
    fn from(_: &NoWhispers) -> Self {
        ModifierType::NoWhispers
    }
}

impl ModifierTrait for NoWhispers {}
