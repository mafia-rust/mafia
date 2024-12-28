use serde::Serialize;

use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{
    common_role, AbilitySelection, AvailableAbilitySelection, ControllerID,
    ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl
};


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

            actor_ref.set_role_state(game, Necromancer{
                used_bodies,
                currently_used_player: Some(currently_used_player)
            })
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Necromancer, 0),
            AvailableAbilitySelection::new_two_player_option(
                PlayerReference::all_players(game)
                    .filter(|p|!p.alive(game))
                    .filter(|target|
                        (self.used_bodies.iter().filter(|p| **p == *target).count() < 2)
                    )
                    .filter(|p|*p != actor_ref)
                    .collect(),
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .collect(),
                true,
                true
            ),
            AbilitySelection::new_two_player_option(None),
            !actor_ref.alive(game) || Detained::is_detained(game, actor_ref),
            Some(PhaseType::Obituary),
            false, 
            vec_set!(actor_ref)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Necromancer, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, Necromancer { currently_used_player: None, ..self });
        }
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
       vec![
           crate::game::components::insider_group::InsiderGroupID::Mafia
       ].into_iter().collect()
   }
}
impl GetClientRoleState<ClientRoleState> for Necromancer {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}