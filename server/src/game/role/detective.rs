use serde::Serialize;

use crate::game::ability_input::ControllerID;
use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{ControllerParametersMap, Priority, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Detective;

impl RoleStateImpl for Detective {
    type ClientRoleState = Detective;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}
        
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(visit) = actor_visits.first() else {return};
        let target_ref = visit.target;
        let suspicious = if Confused::is_confused(game, actor_ref) {
            Self::player_is_suspicious_confused(game, target_ref, actor_ref)
        }else{
            Detective::player_is_suspicious(game, target_ref)
        };

        let message = ChatMessageVariant::DetectiveResult {
            suspicious
        };
        
        actor_ref.push_night_message(game, message);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Detective, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Detective, 0),
            false
        )
    }
}

impl Detective {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).friends_with_resolution_state(GameConclusion::Town)
        }
    }
    pub fn player_is_suspicious_confused(game: &Game, player_ref: PlayerReference, actor_ref: PlayerReference) -> bool {
        player_ref.night_framed(game) ||
        Confused::is_red_herring(game, actor_ref, player_ref)
    }
}