use super::ModifierTrait;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct DeadCanChat;

impl ModifierTrait for DeadCanChat{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::DeadCanChat
    }
}
