use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::PlayerReference};
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{same_evil_team, Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;


pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    type RoleActionChoice = super::common_role::CommonRoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Deception {return;}
    
        let framer_visits = actor_ref.night_visits(game).clone();


        let Some(first_visit) = framer_visits.first() else {return};

        first_visit.target.set_night_framed(game, true);

        let Some(second_visit) = framer_visits.get(1) else {return};
    
        if !first_visit.target.night_jailed(game) {
            first_visit.target.set_night_appeared_visits(game, Some(vec![
                Visit{ target: second_visit.target, attack: false }
            ]));
        }

        actor_ref.set_night_visits(game, vec![first_visit.clone()]);
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        (
            actor_ref.selection(game).is_empty() &&
            actor_ref != target_ref &&
            target_ref.alive(game) &&
            !same_evil_team(game, actor_ref, target_ref)
        ) || 
        (
            actor_ref.selection(game).len() == 1
        )
        
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], attack: false }, 
                Visit{ target: target_refs[1], attack: false }
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit{ target: target_refs[0], attack: false }
            ]
        } else {
            Vec::new()
        }
    }
}
