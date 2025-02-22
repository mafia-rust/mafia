
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Doctor {
    self_heals_remaining: u8,
    target_healed_ref: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    self_heals_remaining: u8
}

impl Default for Doctor {
    fn default() -> Self {
        Self { 
            self_heals_remaining: 1,
            target_healed_ref: None
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Doctor {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Heal => {
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;

                target_ref.increase_defense_to(game, DefensePower::Protection);

                if actor_ref == target_ref{
                    actor_ref.set_role_state(game, RoleState::Doctor(Doctor{
                        self_heals_remaining: self.self_heals_remaining.saturating_sub(1), 
                        target_healed_ref: Some(target_ref)
                    }));
                }else{
                    actor_ref.set_role_state(game, RoleState::Doctor(Doctor{
                        target_healed_ref: Some(target_ref),
                        ..self
                    }));
                }

            }
            Priority::Investigative => {
                if let Some(target_healed_ref) = self.target_healed_ref {
                    if target_healed_ref.night_attacked(game){
                        
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_healed_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            self.self_heals_remaining > 0,
            true,
            false,
            ControllerID::role(actor_ref, Role::Doctor, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Doctor, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Doctor{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_ref: None
        });
    }
}impl GetClientRoleState<ClientRoleState> for Doctor {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            self_heals_remaining: self.self_heals_remaining
        }
    }
}