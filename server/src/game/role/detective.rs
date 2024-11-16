use serde::Serialize;

use crate::game::ability_input::common_selection::one_player_option_selection::AvailableOnePlayerOptionSelection;
use crate::game::components::confused::Confused;
use crate::game::components::generic_ability::{AvailableGenericAbilitySelection, AvailableGenericAbilitySelectionType};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_map::VecMap;
use crate::vec_set::VecSet;

use super::{Priority, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Detective;

impl RoleStateImpl for Detective {
    type ClientRoleState = Detective;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
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
    fn available_generic_ability_selection(self, game: &Game, actor_ref: PlayerReference) -> AvailableGenericAbilitySelection {
        let mut all_allowed_inputs = VecSet::new();
        all_allowed_inputs.insert(None);
        for player in PlayerReference::all_players(game) {
            if crate::game::role::common_role::can_night_select(game, actor_ref, player){
                all_allowed_inputs.insert(Some(player));
            }
        }
        
        
        let mut map = VecMap::new();
        map.insert(0, 
            AvailableGenericAbilitySelectionType::OnePlayerOptionSelection{
                selection: AvailableOnePlayerOptionSelection(all_allowed_inputs)
            }
        );
        
        AvailableGenericAbilitySelection::new(map)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_generic_ability_to_visits(game, actor_ref, 0, false)
    }
}

impl Detective {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).can_win_when_resolution_state_reached(GameConclusion::Town)
        }
    }
}