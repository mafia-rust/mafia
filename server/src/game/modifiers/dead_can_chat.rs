use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct DeadCanChat;

/*
    There is modifier specific code in the common_role::get_send_chat_groups() function
*/
impl From<&DeadCanChat> for ModifierType{
    fn from(_: &DeadCanChat) -> Self {
        ModifierType::DeadCanChat
    }
}

impl ModifierTrait for DeadCanChat{}