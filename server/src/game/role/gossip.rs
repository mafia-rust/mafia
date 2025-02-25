use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::detective::Detective;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleState, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Gossip {
    red_herring: Option<PlayerReference>,
}

impl RoleStateImpl for Gossip {
    type ClientRoleState = Gossip;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            
            let enemies = self.enemies(game, visit.target, actor_ref);
            let message = ChatMessageVariant::GossipResult{ enemies };
            
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

    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        actor_ref.set_role_state(game, RoleState::Gossip(Gossip{
            red_herring: PlayerReference::generate_red_herring(actor_ref, game)
        }));
    }
}

impl Gossip {
    pub fn enemies(self, game: &Game, player_ref: PlayerReference, actor_ref: PlayerReference) -> bool {
        match player_ref.night_appeared_visits(game) {
            Some(x) => x.clone(),
            None => player_ref.all_night_visits_cloned(game),
        }
            .iter()
            .map(|v|v.target.clone())
            .any(
                |targets_target: PlayerReference|   
                    if Confused::is_confused(game, actor_ref) {
                        targets_target.night_framed(game) || self.red_herring.is_some_and(|red_herring| red_herring == targets_target)
                    } else {
                        Detective::player_is_suspicious(game, targets_target)
                    }
            )
    }
}