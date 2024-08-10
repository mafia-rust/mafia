use serde::Serialize;

use crate::game::{chat::ChatGroup, components::love_linked::LoveLinked};
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
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
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
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
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
