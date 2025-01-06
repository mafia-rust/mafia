use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ScheduledNominations;

/*
    There is modifier specific code in the common_role::get_send_chat_groups() function
*/
impl From<&ScheduledNominations> for ModifierType{
    fn from(_: &ScheduledNominations) -> Self {
        ModifierType::ScheduledNominations
    }
}

impl ModifierTrait for ScheduledNominations{}