use serde::Serialize;

use crate::game::ability_input::AvailableTwoPlayerOptionSelection;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::components::transporting::transport;
use crate::vec_set;

use crate::vec_map::VecMap;

use super::{common_role, ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Transporter;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Transporter {
    type ClientRoleState = Transporter;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if priority != OnMidnightPriority::Transporter {return;}
    
        let transporter_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(first_visit) = transporter_visits.get(0).map(|v| v.target) else {return};
        let Some(second_visit) = transporter_visits.get(1).map(|v| v.target) else {return};
        
        transport(
            &actor_ref, game, midnight_variables,
            &VecMap::new_from_vec(vec![(first_visit, second_visit), (second_visit, first_visit)]), true, &|_| true
        );
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let available_players: vec_set::VecSet<PlayerReference> = PlayerReference::all_players(game)
            .filter(|p| p.alive(game))
            .collect();

        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Transporter, 0))
            .available_selection(AvailableTwoPlayerOptionSelection {
                available_first_players: available_players.clone(),
                available_second_players: available_players,
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
            ControllerID::role(actor_ref, Role::Transporter, 0),
            false
        )
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}