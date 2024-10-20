use serde::Serialize;

use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{RevealedGroupID, Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Cupid;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cupid {
    type ClientRoleState = Cupid;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Cupid => {
                let visits = actor_ref.night_visits(game);

                let Some(first_visit) = visits.get(0) else {return};
                let Some(second_visit) = visits.get(1) else {return};
                
                let player1 = first_visit.target;
                let player2 = second_visit.target;

                LoveLinked::add_love_link(game, player1, player2);
            },
            _ => ()
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let selection = actor_ref.selection(game);

        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref != target_ref &&
        ((
            selection.is_empty()
        ) || (
            selection.len() == 1 &&
            Some(target_ref) != selection.first().copied()
        )) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        !RevealedGroupID::players_in_same_revealed_group(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], attack: false },
                Visit{ target: target_refs[1], attack: false }
            ]
        } else {
            Vec::new()
        }
    }
    fn default_revealed_groups(self) -> std::collections::HashSet<crate::game::components::revealed_group::RevealedGroupID> {
        vec![
            crate::game::components::revealed_group::RevealedGroupID::Mafia
        ].into_iter().collect()
    }
}
