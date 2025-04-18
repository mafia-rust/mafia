
use serde::Serialize;

use crate::game::ability_input::selection_type::two_role_option_selection::TwoRoleOptionSelection;
use crate::game::ability_input::{AvailableTwoRoleOptionSelection, ControllerID};
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::Game;
use super::{ControllerParametersMap, GetClientRoleState, Role, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Steward {
    self_heals_remaining: u8,
    target_healed_refs: Vec<PlayerReference>,
    previous_input: TwoRoleOptionSelection
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    steward_protects_remaining: u8,
    previous_role_chosen: TwoRoleOptionSelection
}

impl Default for Steward {
    fn default() -> Self {
        Self { 
            self_heals_remaining: 1,
            target_healed_refs: vec![],
            previous_input: TwoRoleOptionSelection(None, None)
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Steward {
    type ClientRoleState = ClientRoleState;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {

        if actor_ref.night_blocked(midnight_variables) {return}
        if actor_ref.ability_deactivated_from_death(game) {return}

        match priority {
            OnMidnightPriority::Heal => {
                let mut healed_players = vec![];
                let Some(selection) = 
                    ControllerID::role(actor_ref, Role::Steward, 0)
                        .get_two_role_option_selection(game)
                        .cloned()
                        else {return};
                let TwoRoleOptionSelection(first, second) = selection;
                
                if let Some(role) = first {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, midnight_variables, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                if let Some(role) = second {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, midnight_variables, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                
                let self_heals_remaining = if 
                    first.is_some_and(|r|r == Role::Steward) || 
                    second.is_some_and(|r|r == Role::Steward)
                {
                    self.self_heals_remaining.saturating_sub(1)
                }else{
                    self.self_heals_remaining
                };
                
                actor_ref.set_role_state(game, Steward{
                    self_heals_remaining,
                    target_healed_refs: healed_players,
                    previous_input: TwoRoleOptionSelection(first, second), //updates here
                });
            }
            OnMidnightPriority::DeleteMessages => {
                for target_healed_ref in self.target_healed_refs{
                    if target_healed_ref.night_attacked(midnight_variables){

                        target_healed_ref.set_night_messages(midnight_variables, 
                            target_healed_ref.night_messages(midnight_variables)
                                .iter()
                                .filter(|m|
                                    !matches!(m,ChatMessageVariant::YouWereProtected|ChatMessageVariant::YouSurvivedAttack)
                                )
                                .cloned()
                                .collect()
                        );
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Steward, 0))
            .available_selection(AvailableTwoRoleOptionSelection {
                available_roles: Role::values()
                    .into_iter()
                    .filter(|role| self.self_heals_remaining>0 || role != &Role::Steward)
                    .filter(|role| self.previous_input.0 != Some(*role) && self.previous_input.1 != Some(*role))
                    .map(Some)
                    .chain(std::iter::once(None))
                    .collect(),
                can_choose_duplicates: false
            })
            .night_typical(actor_ref)
            .build_map()
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Steward{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_refs: vec![],
            previous_input: self.previous_input
        });
    }
}
impl GetClientRoleState<ClientRoleState> for Steward {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            steward_protects_remaining: self.self_heals_remaining,
            previous_role_chosen: self.previous_input
        }
    }
}