use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Snoop;

impl RoleStateImpl for Snoop {
    type ClientRoleState = Snoop;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}


        let actor_visits = actor_ref.night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){

            let townie = if Confused::is_confused(game, actor_ref) {
                false
            }else{
                visit.target.win_condition(game).is_loyalist_for(GameConclusion::Town) &&
                    actor_ref.all_visitors(game).len() == 0 &&
                    !visit.target.has_suspicious_aura(game)
            };

            actor_ref.push_night_message(game, 
                ChatMessageVariant::SnoopResult { townie }
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