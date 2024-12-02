use serde::Serialize;

use crate::game::ability_input::AbilityID;
use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{AllPlayersAvailableAbilities, Priority, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Detective;

impl RoleStateImpl for Detective {
    type ClientRoleState = Detective;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let suspicious = if Confused::is_confused(game, actor_ref) {
                false
            }else{
                Detective::player_is_suspicious(game, visit.target)
            };

            let message = ChatMessageVariant::SheriffResult {
                suspicious
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn available_abilities(self, game: &Game, actor_ref: PlayerReference) -> AllPlayersAvailableAbilities {
        crate::game::role::common_role::available_abilities_one_player_night(game, actor_ref, false, AbilityID::role(actor_ref.role(game), 0))
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_saved_ability_to_visits(game, actor_ref, AbilityID::role(actor_ref.role(game), 0), false)
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
}