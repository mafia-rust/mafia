use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct TwoThirdsMajority;

impl From<&TwoThirdsMajority> for ModifierType{
    fn from(_: &TwoThirdsMajority) -> Self {
        ModifierType::TwoThirdsMajority
    }
}

impl ModifierTrait for TwoThirdsMajority {}
