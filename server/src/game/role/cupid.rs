use serde::Serialize;

use crate::game::attack_type::AttackData;
use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::{common_role, AvailableAbilitySelection, ControllerID, ControllerParametersMap, InsiderGroupID, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Cupid;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cupid {
    type ClientRoleState = Cupid;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Cupid => {
                let visits = actor_ref.untagged_night_visits_cloned(game);

                let Some(first_visit) = visits.get(0) else {return};
                let Some(second_visit) = visits.get(1) else {return};
                
                let player1 = first_visit.target;
                let player2 = second_visit.target;

                LoveLinked::add_love_link(game, player1, player2);
            },
            _ => ()
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {

        let available_players: vec_set::VecSet<PlayerReference> = PlayerReference::all_players(game)
            .into_iter()
            .filter(|p|
                p.alive(game) &&
                *p != actor_ref &&
                !InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
            )
            .collect();

        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Cupid, 0),
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
            ControllerID::role(actor_ref, Role::Cupid, 0),
            false
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn attack_data(&self, game: &Game, actor_ref: PlayerReference) ->AttackData {
        AttackData::reliant(game, actor_ref)
    }
}
