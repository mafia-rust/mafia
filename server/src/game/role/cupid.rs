use serde::Serialize;

use crate::game::components::love_linked::LoveLinked;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{same_evil_team, Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Cupid;

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Cupid {
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

        !actor_ref.night_jailed(game) &&
        actor_ref != target_ref &&
        ((
            selection.is_empty()
        ) || (
            selection.len() == 1 &&
            Some(target_ref) != selection.first().copied()
        )) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        !same_evil_team(game, actor_ref, target_ref)
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
}
