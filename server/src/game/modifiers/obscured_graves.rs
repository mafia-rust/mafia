use crate::game::{grave::{GraveInformation, GraveReference}, Game};

use super::ModifierTrait;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ObscuredGraves;

impl ModifierTrait for ObscuredGraves{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::ObscuredGraves
    }
    fn on_grave_added(self, game: &mut Game, grave: GraveReference) {
        grave.deref_mut(game).information = GraveInformation::Obscured;
    }
}
