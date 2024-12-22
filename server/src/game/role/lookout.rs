use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Serialize, Debug, Default)]
pub struct Lookout;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Lookout {
    type ClientRoleState = Lookout;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            
            let mut seen_players: Vec<PlayerReference> = visit.target.all_appeared_visitors(game).into_iter().filter(|p|actor_ref!=*p).collect();
            seen_players.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::LookoutResult { players:
                PlayerReference::ref_vec_to_index(seen_players.as_slice())
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            ControllerID::role(actor_ref, Role::Lookout, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Lookout, 0),
            false
        )
    }
}