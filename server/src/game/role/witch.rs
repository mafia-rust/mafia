use serde::Serialize;

use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{same_evil_team, CustomClientRoleState, Priority, RoleState, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct Witch{
    currently_used_player: Option<PlayerReference> 
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

impl RoleStateImpl<ClientRoleState> for Witch {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, RoleState::Witch(Witch{
                currently_used_player: Some(currently_used_player)
            }))
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        ((
            actor_ref.selection(game).is_empty() &&
            !same_evil_team(game, actor_ref, target_ref)
        ) || (
            actor_ref.selection(game).len() == 1
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
            actor_ref.set_role_state(game, RoleState::Witch(Witch { currently_used_player: None }));
        }
    }
}

impl CustomClientRoleState<ClientRoleState> for Witch {
    fn get_client_role_state(self, _: &Game, _: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}