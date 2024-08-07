use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::grave::{Grave, GraveReference};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Minion{
    currently_used_player: Option<PlayerReference> 
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Minion {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, RoleState::Minion(Minion{
                currently_used_player: Some(currently_used_player)
            }))
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        ((
            actor_ref != target_ref &&
            actor_ref.selection(game).is_empty()
        ) || (
            actor_ref.selection(game).len() == 1
        ))
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{target: target_refs[0], attack: false}, 
                Visit{target: target_refs[1], attack: false},
            ]
        }else{
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, _actor_ref: PlayerReference) -> bool {
        PlayerReference::all_players(game).filter(|player_ref|{
            player_ref.alive(game) && player_ref.role(game).faction() == Faction::Town
        }).count() == 0
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if actor_ref.get_won_game(game) && actor_ref.alive(game) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Minion(Minion { currently_used_player: None }));
        }
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