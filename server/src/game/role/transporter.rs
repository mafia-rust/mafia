use serde::Serialize;

use crate::game::components::detained::Detained;
use crate::game::attack_power::DefensePower;
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
        
        
        actor_ref.transport(game, first_visit.target, second_visit.target);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {

        let available_players: vec_set::VecSet<PlayerReference> = PlayerReference::all_players(game)
            .into_iter()
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
}