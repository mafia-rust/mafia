use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::detective::Detective;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Gossip;

impl RoleStateImpl for Gossip {
    type ClientRoleState = Gossip;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if priority != OnMidnightPriority::Investigative {return;}

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let enemies = Self::visited_enemies(game, midnight_variables, visit.target, actor_ref);
            let message: ChatMessageVariant = ChatMessageVariant::GossipResult{ enemies };
            actor_ref.push_night_message(midnight_variables, message);
        }
        
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Gossip, 0))
            .single_player_selection_typical(actor_ref, false, true)
            .night_typical(actor_ref)
            .add_grayed_out_condition(false)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Gossip, 0),
            false
        )
    }
}

impl Gossip {
    pub fn visited_enemies(game: &Game, midnight_variables: &MidnightVariables, player_ref: PlayerReference, actor_ref: PlayerReference) -> bool {
        let is_confused = Confused::is_confused(game, actor_ref);
        match player_ref.night_appeared_visits(midnight_variables) {
            Some(x) => x.clone(),
            None => player_ref.all_night_visits_cloned(game),
        }
            .into_iter()
            .any(|visit: Visit|
                if is_confused {
                    Detective::player_is_suspicious_confused(game, midnight_variables, visit.target, actor_ref)
                } else {
                    Detective::player_is_suspicious(game, midnight_variables, visit.target)
                }
              )
   }
}