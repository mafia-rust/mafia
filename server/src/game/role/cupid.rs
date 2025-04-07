use serde::Serialize;

use crate::game::ability_input::AvailableTwoPlayerOptionSelection;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::{common_role, ControllerID, ControllerParametersMap, InsiderGroupID, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Cupid;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cupid {
    type ClientRoleState = Cupid;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Cupid => {
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
            .filter(|p|
                p.alive(game) &&
                *p != actor_ref &&
                !InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
            )
            .collect();

        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Cupid, 0))
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
            ControllerID::role(actor_ref, Role::Cupid, 0),
            false
        )
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
