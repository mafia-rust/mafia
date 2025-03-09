use serde::Serialize;

use crate::game::ability_input::ControllerID;
use crate::game::attack_power::DefensePower;
use crate::game::components::confused::Confused;
use crate::game::chat::ChatMessageVariant;
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::{common_role, Priority, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Supervisor;


impl RoleStateImpl for Supervisor {
    type ClientRoleState = Supervisor;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_blocked(game) {return}
        if actor_ref.ability_deactivated_from_death(game) {return}
        if priority != Priority::Investigative {return;}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(visit) = actor_visits.first() else {return};
        
        let mut visitors = visit.target.all_appeared_visitors(game).len().clamp(0, u8::MAX as usize) as u8; //includes yourself
        let mut visited = visit.target.tracker_seen_visits(game).len().clamp(0, u8::MAX as usize) as u8;
        if Confused::is_confused(game, actor_ref){
            //add or subtract 1 randomly from the counts
            match rand::random_range(0..3u8) {
                0 => (),
                1 => visitors = visitors.saturating_add(1).min(game.num_players()),
                2 => visitors = visitors.saturating_sub(1).min(1u8), //includes yourself
                _ => unreachable!("This is the most confident I've been while writing and unreachable. If you see this message feel free to call me a dumbass."),
            }
            match rand::random_range(0..4u8) {
                0 => (),
                1 => visited = visited.saturating_add(1).min(game.num_players()),
                2|3 => visited = visited.saturating_sub(1),
                _ => unreachable!("This is the most confident I've been while writing and unreachable. If you see this message feel free to call me a dumbass."),
            }
        }

        let message = ChatMessageVariant::SupervisorResult{ visited, visitors };
        actor_ref.push_night_message(game, message);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> crate::game::ability_input::ControllerParametersMap {
        common_role::controller_parameters_map_player_list_night_typical(
            game, 
            actor_ref, 
            false, 
            true, 
            false, 
            ControllerID::role(actor_ref, Role::Supervisor, 0),
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<crate::game::visit::Visit> {
        common_role::convert_controller_selection_to_visits(
            game, 
            actor_ref, 
            ControllerID::role(actor_ref, Role::Supervisor, 0),
            false,
        )
    }
}

impl Supervisor {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town)
        }
    }
}