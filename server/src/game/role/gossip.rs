use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::detective::Detective;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Gossip;

impl RoleStateImpl for Gossip {
    type ClientRoleState = Gossip;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            
            let enemies = Self::visited_enemies(game, visit.target, actor_ref);
            let message: ChatMessageVariant = ChatMessageVariant::GossipResult{ enemies };
            actor_ref.push_night_message(game, message);
        }
        
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Gossip, 0)
        )
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
    pub fn visited_enemies(game: &Game, player_ref: PlayerReference, actor_ref: PlayerReference) -> bool {
        match player_ref.night_appeared_visits(game) {
            Some(x) => x.clone(),
            None => player_ref.all_night_visits_cloned(game),
        }
            .iter()
            .any(|visit: &Visit|
                if Confused::is_confused(game, actor_ref) {
                    Detective::player_is_suspicious_confused(game, visit.target, actor_ref)
                } else {
                    Detective::player_is_suspicious(game, visit.target)
                }
            )
    }
}