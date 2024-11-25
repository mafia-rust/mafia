use serde::Serialize;

use crate::game::grave::{GraveInformation, GraveReference};
use crate::game::phase::PhaseType;
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, Game};
use crate::game::player::PlayerReference;

use super::{GetClientRoleState, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct CrossingGuard{
    //when swapping roles back
    player_to_swap_with: Option<PlayerReference>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState;

impl GetClientRoleState<ClientRoleState> for CrossingGuard {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}

impl RoleStateImpl for CrossingGuard {
    type ClientRoleState = ClientRoleState;
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let Some(grave) = GraveReference::all(game).into_iter()
            .find(|grave_ref| grave_ref.deref(game).player == target_ref) else {return false};
        let GraveInformation::Normal { role, .. } = 
            grave.deref(game).clone().information else {return false};
        
        if !RoleSet::Town.get_roles().contains(&role) {return false};

        actor_ref != target_ref && actor_ref.alive(game)
    }
    fn do_day_action(mut self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if self.player_to_swap_with.is_some() {return};

        let Some(GraveInformation::Normal { role, .. }) = GraveReference::all(game).into_iter()
            .find(|grave_ref| grave_ref.deref(game).player == target_ref)
            .map(|grave|grave.deref(game).information.clone()) else {return};

        let other_role = if role == target_ref.role(game){
            target_ref.role_state(game).clone()
        }else{
            role.default_state()
        };
        self.player_to_swap_with = Some(actor_ref);

        actor_ref.set_role(game, other_role);
        target_ref.set_role(game, self);
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase != PhaseType::Discussion {return};
        let Some(player_to_swap_with) = self.player_to_swap_with else {return};
        
        self.player_to_swap_with = None;
        let other_role = player_to_swap_with.role_state(game).clone();

        actor_ref.set_role(game, other_role);
        player_to_swap_with.set_role(game, self);
    }
}