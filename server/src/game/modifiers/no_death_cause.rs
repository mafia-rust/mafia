use crate::game::{grave::{GraveInformation, GraveReference}, Game};

use super::ModifierTrait;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct NoDeathCause;

impl ModifierTrait for NoDeathCause{
    fn modifier_type(&self) -> super::ModifierType {
        super::ModifierType::NoDeathCause
    }
    fn on_grave_added(self, game: &mut Game, grave: GraveReference) {
        match grave.deref(game).information.clone() {
            GraveInformation::Obscured => {},
            GraveInformation::Normal { role, will, death_notes, .. } => {
                grave.deref_mut(game).information = GraveInformation::Normal{
                    role,
                    will,
                    death_cause: crate::game::grave::GraveDeathCause::Execution,
                    death_notes
                }
            },
        }
    }
}
