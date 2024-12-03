
use serde::Serialize;

use crate::game::ability_input::selection_type::two_role_option_selection::TwoRoleOptionSelection;
use crate::game::ability_input::{ControllerID, AbilityInput};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Steward {
    self_heals_remaining: u8,
    target_healed_refs: Vec<PlayerReference>,
    role_chosen: TwoRoleOptionSelection,
    previous_input: TwoRoleOptionSelection
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    steward_protects_remaining: u8,
    role_chosen: TwoRoleOptionSelection,
    previous_role_chosen: TwoRoleOptionSelection
}

impl Default for Steward {
    fn default() -> Self {
        Self { 
            self_heals_remaining: 1,
            target_healed_refs: vec![],
            role_chosen: TwoRoleOptionSelection(None, None),
            previous_input: TwoRoleOptionSelection(None, None)
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Steward {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if actor_ref.night_blocked(game) {return}
        if !actor_ref.alive(game) {return}

        match priority {
            Priority::Heal => {
                let mut healed_players = vec![];
                let mut healed_roles = self.role_chosen;
                
                if self.self_heals_remaining == 0 && healed_roles.contains(Role::Steward) {
                    healed_roles = TwoRoleOptionSelection(None, None)
                }
                if healed_roles.any_in_common(&self.previous_input) || healed_roles.same_role(){
                    healed_roles = TwoRoleOptionSelection(None, None)
                }

                if let Some(role) = healed_roles.0 {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                if let Some(role) = healed_roles.1 {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                
                let self_heals_remaining = if healed_roles.contains(Role::Steward) {
                    self.self_heals_remaining.saturating_sub(1)
                }else{
                    self.self_heals_remaining
                };
                
                actor_ref.set_role_state(game, Steward{
                    self_heals_remaining,
                    target_healed_refs: healed_players,
                    role_chosen: healed_roles.clone(),
                    previous_input: healed_roles, //updates here
                });
            }
            Priority::Investigative => {
                for target_healed_ref in self.target_healed_refs{
                    if target_healed_ref.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_healed_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn on_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: AbilityInput) {
        if actor_ref != input_player {return}
        if !actor_ref.alive(game) {return}
        
        let Some(selection) = ability_input
            .get_two_role_option_selection_if_id(ControllerID::role(actor_ref.role(game), 0)) else {return};
        
        if selection.any_in_common(&self.previous_input) || selection.same_role(){
            return;
        }

        if self.self_heals_remaining == 0 && selection.contains(Role::Steward){
            return;
        }

        actor_ref.set_role_state(game, Steward{
            role_chosen: selection,
            ..self
        });
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Steward{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_refs: vec![],
            role_chosen: TwoRoleOptionSelection(None, None),
            previous_input: self.previous_input
        });
    }
}
impl GetClientRoleState<ClientRoleState> for Steward {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            steward_protects_remaining: self.self_heals_remaining,
            role_chosen: self.role_chosen,
            previous_role_chosen: self.previous_input
        }
    }
}