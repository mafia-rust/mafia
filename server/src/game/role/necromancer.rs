use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Necromancer { 
    used_bodies: Vec<PlayerReference>, 
    currently_used_player: Option<PlayerReference> 
}
impl RoleStateImpl for Necromancer {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            let mut used_bodies = self.used_bodies;
            used_bodies.push(currently_used_player);

            actor_ref.set_role_state(game, RoleState::Necromancer(Necromancer{
                used_bodies,
                currently_used_player: Some(currently_used_player)
            }))
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        ((
            actor_ref.selection(game).is_empty() &&
            !target_ref.alive(game) &&
            !self.used_bodies.iter().any(|p| *p == target_ref)
        ) || (
            actor_ref != target_ref &&
            actor_ref.selection(game).len() == 1 &&
            target_ref.alive(game)
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
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Necromancer(Necromancer { used_bodies: self.used_bodies, currently_used_player: None }));
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
