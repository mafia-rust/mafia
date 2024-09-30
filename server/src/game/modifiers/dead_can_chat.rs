use super::ModifierTrait;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct DeadCanChat;

/*
    There is modifier specific code in the common_role::get_send_chat_groups() function
*/

impl ModifierTrait for DeadCanChat{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::DeadCanChat
    }
}
