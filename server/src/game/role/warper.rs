use serde::Serialize;

use crate::game::grave::Grave;
use crate::game::phase::PhaseType;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Warper;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Warper {
    type ClientRoleState = Warper;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Warper {return;}
    
        let transporter_visits = actor_ref.untagged_night_visits_cloned(game).clone();
        let Some(first_visit) = transporter_visits.get(0) else {return};
        let Some(second_visit) = transporter_visits.get(1) else {return};
        
        
        first_visit.target.push_night_message(game, ChatMessageVariant::Transported);
        actor_ref.push_night_message(game, ChatMessageVariant::TargetHasRole { role: first_visit.target.role(game) });
    
        for player_ref in PlayerReference::all_players(game){
            if player_ref == actor_ref {continue;}
            if player_ref.role(game) == Role::Warper {continue;}
            if player_ref.role(game) == Role::Transporter {continue;}

            let new_visits = player_ref.all_night_visits_cloned(game).clone().into_iter().map(|mut v|{
                if v.target == first_visit.target {
                    v.target = second_visit.target;
                }
                v
            }).collect();
            player_ref.set_night_visits(game, new_visits);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        ((
            actor_ref != target_ref &&
            actor_ref.selection(game).is_empty()
        ) || (
            actor_ref.selection(game).len() == 1
        ))
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    WinCondition::are_friends(&p.win_condition(game), actor_ref.win_condition(game))
                )

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
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