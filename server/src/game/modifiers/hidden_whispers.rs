use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct HiddenWhispers;

impl From<&HiddenWhispers> for ModifierType{
    fn from(_: &HiddenWhispers) -> Self {
        ModifierType::HiddenWhispers
    }
}

impl ModifierTrait for HiddenWhispers {}