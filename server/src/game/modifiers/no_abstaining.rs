use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoAbstaining;

/*
    There is modifier specific code in the set_verdict() function
*/
impl From<&NoAbstaining> for ModifierType{
    fn from(_: &NoAbstaining) -> Self {
        ModifierType::NoAbstaining
    }
}
impl ModifierTrait for NoAbstaining{
}
