use super::{ModifierTrait, ModifierType};
/* See Godfather for the actual implementation*/
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct BackupGetsGun;

impl From<&BackupGetsGun> for ModifierType{
    fn from(_: &BackupGetsGun) -> Self {
        ModifierType::PlayerDrops
    }
}

impl ModifierTrait for BackupGetsGun{}
