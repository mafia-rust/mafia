
use serde::Serialize;

use crate::game::ability_input::selection_type::two_role_option_selection::TwoRoleOptionSelection;
use crate::game::ability_input::ControllerID;
use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::Game;
use crate::vec_set;
use super::{AbilitySelection, AvailableAbilitySelection, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl};

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
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if actor_ref.night_blocked(game) {return}
        if actor_ref.ability_deactivated_from_death(game) {return}

        match priority {
            Priority::Heal => {
                let mut healed_players = vec![];
                let selection = game.saved_controllers.get_controller_current_selection_two_role_option(ControllerID::role(actor_ref, Role::Steward, 0));
                let Some(selection) = selection else {return};
                let TwoRoleOptionSelection(first, second) = selection;
                
                if let Some(role) = first {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                if let Some(role) = second {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, DefensePower::Protection);
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
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        let valid_roles = Role::values()
            .into_iter()
            .filter(|role| self.self_heals_remaining>0 || role != &Role::Steward)
            .filter(|role| self.previous_input.0 != Some(*role) && self.previous_input.1 != Some(*role))
            .map(Some)
            .chain(std::iter::once(None))
            .collect();
            
        
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Steward, 0),
            AvailableAbilitySelection::new_two_role_option(valid_roles, false),
            AbilitySelection::new_two_role_option(None, None),
            actor_ref.ability_deactivated_from_death(game) || Detained::is_detained(game, actor_ref),
            Some(PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Steward{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_refs: vec![],
            previous_input: self.previous_input
        });
    }
    fn attack_data(&self, _game: &Game, _actor_ref: PlayerReference) -> crate::game::attack_type::AttackData {
        crate::game::attack_type::AttackData::none()
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