use serde::Serialize;

use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{GetClientRoleState, Priority, RoleState, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct Necromancer { 
    used_bodies: Vec<PlayerReference>, 
    currently_used_player: Option<PlayerReference> 
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

impl RoleStateImpl for Necromancer {
    type ClientRoleState = ClientRoleState;
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
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
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
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Necromancer(Necromancer { used_bodies: self.used_bodies, currently_used_player: None }));
        }
    }
    fn default_revealed_groups(self) -> std::collections::HashSet<crate::game::components::revealed_group::RevealedGroupID> {
        vec![
            crate::game::components::revealed_group::RevealedGroupID::Mafia
        ].into_iter().collect()
    }
}
impl GetClientRoleState<ClientRoleState> for Necromancer {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}