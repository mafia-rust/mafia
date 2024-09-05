use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::resolution_state::ResolutionState;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Snoop;

impl RoleStateImpl for Snoop {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){

            actor_ref.push_night_message(game, 
                ChatMessageVariant::SnoopResult { townie: 
                    ResolutionState::requires_only_this_resolution_state(game, visit.target, ResolutionState::Town) &&
                    actor_ref.all_visitors(game).len() == 0 &&
                    !visit.target.has_suspicious_aura(game)
                }
            );
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
}