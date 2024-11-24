use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                let framer_visits = actor_ref.night_visits_cloned(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                first_visit.target.set_night_framed(game, true);

                let Some(second_visit) = framer_visits.get(1) else {return};
                second_visit.target.set_night_framed(game, true);
            
                first_visit.target.set_night_appeared_visits(game, Some(vec![
                    Visit::new_none(first_visit.target, second_visit.target, false)
                ]));
                
                actor_ref.set_night_appeared_visits(game, Some(vec![
                    Visit::new_none(actor_ref, first_visit.target, false)
                ]));
            }
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        (
            actor_ref.selection(game).is_empty() &&
            actor_ref != target_ref &&
            target_ref.alive(game)
        ) || 
        (
            actor_ref.selection(game).len() == 1
        )
        
    }
    fn convert_selection_to_visits(self, _game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false), 
                Visit::new_none(actor_ref, target_refs[1], false)
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false)
            ]
        } else {
            Vec::new()
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
