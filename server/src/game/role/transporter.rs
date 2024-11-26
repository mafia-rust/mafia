use serde::Serialize;

use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Transporter;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Transporter {
    type ClientRoleState = Transporter;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Transporter {return;}
    
        let transporter_visits = actor_ref.untagged_night_visits_cloned(game).clone();
        let Some(first_visit) = transporter_visits.get(0) else {return};
        let Some(second_visit) = transporter_visits.get(1) else {return};
        
        
        first_visit.target.push_night_message(game, ChatMessageVariant::Transported);
        second_visit.target.push_night_message(game, ChatMessageVariant::Transported);
    
        for player_ref in PlayerReference::all_players(game){
            if player_ref == actor_ref {continue;}
            if player_ref.role(game) == Role::Transporter {continue;}


            let new_visits = player_ref.untagged_night_visits_cloned(game).clone().into_iter().map(|mut v|{
                if v.target == first_visit.target {
                    v.target = second_visit.target;
                } else if v.target == second_visit.target{
                    v.target = first_visit.target;
                }
                v
            }).collect();
            player_ref.set_night_visits(game, new_visits);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let chosen_targets = actor_ref.selection(game);

        !Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) && 
        ((
            chosen_targets.is_empty()
        ) || (
            chosen_targets.len() == 1 &&
            Some(target_ref) != chosen_targets.first().copied()
        ))
    }
    fn convert_selection_to_visits(self, _game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false),
                Visit::new_none(actor_ref, target_refs[1], false)
            ]
        } else {
            Vec::new()
        }
    }
}