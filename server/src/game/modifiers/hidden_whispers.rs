use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct HiddenWhispers;

/*
    There is modifier specific code in the player_send_packet::send_chat_messages() function
*/
impl From<&HiddenWhispers> for ModifierType{
    fn from(_: &HiddenWhispers) -> Self {
        ModifierType::HiddenWhispers
    }
}

impl ModifierTrait for HiddenWhispers {}