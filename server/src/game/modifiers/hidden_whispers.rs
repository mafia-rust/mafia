use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct HiddenWhispers;

/*
    There is modifier specific code in the on_client_message::on_client_message() function
    Specifically in the ToServerPacket::SendWhisper branch of the match statement
*/
impl From<&HiddenWhispers> for ModifierType{
    fn from(_: &HiddenWhispers) -> Self {
        ModifierType::HiddenWhispers
    }
}

impl ModifierTrait for HiddenWhispers {}