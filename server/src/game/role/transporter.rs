use serde::Serialize;

use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{common_role, AvailableAbilitySelection, ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

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


            let new_visits = player_ref.all_night_visits_cloned(game).clone().into_iter().map(|mut v|{
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
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {

        let available_players: vec_set::VecSet<PlayerReference> = PlayerReference::all_players(game)
            .filter(|p| p.alive(game))
            .collect();

        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Transporter, 0),
            AvailableAbilitySelection::new_two_player_option(
                available_players.clone(), 
                available_players,
                false,
                true
            ),
            super::AbilitySelection::new_two_player_option(None),
            actor_ref.ability_deactivated_from_death(game) ||
            Detained::is_detained(game, actor_ref),
            Some(crate::game::phase::PhaseType::Obituary),
            false,
            vec_set![actor_ref]
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Transporter, 0),
            false
        )
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}