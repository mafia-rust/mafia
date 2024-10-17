use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Informant;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Informant {
    type ClientRoleState = Informant;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
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
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn default_revealed_groups(self) -> std::collections::HashSet<crate::game::components::revealed_group::RevealedGroupID> {
        vec![
            crate::game::components::revealed_group::RevealedGroupID::Mafia
        ].into_iter().collect()
    }
}

impl Informant {
    pub fn new() -> Self {
        Self{}
    }
}