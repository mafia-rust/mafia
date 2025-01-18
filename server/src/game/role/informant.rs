use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Informant;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Informant {
    type ClientRoleState = Informant;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return}
        

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;

            let mut visited_by: Vec<PlayerReference> =  visit.target.all_appeared_visitors(game).into_iter().filter(|p|actor_ref!=*p).collect();
            visited_by.shuffle(&mut thread_rng());

            let mut visited: Vec<PlayerReference> = target_ref.tracker_seen_visits(game).iter().map(|v|v.target).collect();
            visited.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::InformantResult{
                role: target_ref.role(game), 
                visited_by: PlayerReference::ref_vec_to_index(&visited_by.as_mut_slice()),
                visited: PlayerReference::ref_vec_to_index(visited.as_slice())
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
            false,
            ControllerID::role(actor_ref, Role::Informant, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Informant, 0),
            false
        )
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}

impl Informant {
    pub fn new() -> Self {
        Self{}
    }
}