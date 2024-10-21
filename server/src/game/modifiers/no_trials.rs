use super::ModifierTrait;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoTrials;

/*
    There is modifier specific code in the available_buttons file
*/

impl ModifierTrait for NoTrials{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::NoTrials
    }
}
