use serde::Serialize;

use crate::game::ability_input::AvailableTwoPlayerOptionSelection;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{common_role, ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Porter;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Porter {
    type ClientRoleState = Porter;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if priority != OnMidnightPriority::Warper {return;}
    
        let transporter_visits = actor_ref.untagged_night_visits_cloned(game).clone();
        let Some(first_visit) = transporter_visits.get(0) else {return};
        let Some(second_visit) = transporter_visits.get(1) else {return};
        
        
        first_visit.target.push_night_message(midnight_variables, ChatMessageVariant::Transported);
    
        for player_ref in PlayerReference::all_players(game){
            if player_ref == actor_ref {continue;}
            if player_ref.role(game) == Role::Porter {continue;}
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
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Porter, 0))
            .available_selection(AvailableTwoPlayerOptionSelection {
                available_first_players: PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .filter(|p|*p != actor_ref)
                    .collect(),
                available_second_players:PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .collect(),
                can_choose_duplicates: false,
                can_choose_none: true
            })
            .night_typical(actor_ref)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Porter, 0),
            false
        )
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}