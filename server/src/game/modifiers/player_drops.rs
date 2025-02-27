use super::{ModifierTrait, ModifierType};
/* See Godfather for the actual implementation*/
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct PlayerDrops;

impl From<&PlayerDrops> for ModifierType{
    fn from(_: &PlayerDrops) -> Self {
        ModifierType::PlayerDrops
    }
}

impl ModifierTrait for PlayerDrops{}
