use std::iter::once;

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use crate::game::components::confused::Confused;
use crate::game::role_list::RoleAssignment;
use crate::game::role_outline_reference::RoleOutlineReference;
use crate::game::ability_input::*;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_map::VecMap;
use crate::game::grave::{GraveDeathCause, GraveInformation, GraveKiller, GraveReference};
use crate::weight_map::WeightMap;
use rand::prelude::SliceRandom;
use super::counterfeiter::Counterfeiter;
use super::forger::Forger;
use super::impostor::Impostor;
use super::{common_role, Role, RoleState, RoleStateImpl};
use crate::game::event::on_midnight::OnMidnightPriority;



#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Auditor{
    pub previously_given_results: VecMap<RoleOutlineReference, AuditorResult>,
    pub grave_roles: Vec<Role>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AuditorResult{
    Two{roles: [Role; 2]},
    One{role: Role}
}



pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Auditor {
    type ClientRoleState = Auditor;
    fn on_midnight(mut self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {

        if priority != OnMidnightPriority::Investigative {return;}
        if actor_ref.night_blocked(game) {return;}
        
        let Some(selection) = game.saved_controllers.get_controller_current_selection_two_role_outline_option(
            ControllerID::role(actor_ref, Role::Auditor, 0)
        )
        else{return};

        if let Some(chosen_outline) = selection.0{
            let result = if Confused::is_confused(game, actor_ref){
                Self::get_confused_result(game, chosen_outline, &self.grave_roles)
            }else{
                Self::get_result(game, chosen_outline)
            };
            actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                role_outline: chosen_outline.deref(game).clone(),
                result: result.clone()
            });

            self.previously_given_results.insert(chosen_outline, result);
        }

        if let Some(chosen_outline) = selection.1{
            let result = if Confused::is_confused(game, actor_ref){
                Self::get_confused_result(game, chosen_outline, &self.grave_roles)
            }else{
                Self::get_result(game, chosen_outline)
            };
            actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                role_outline: chosen_outline.deref(game).clone(),
                result: result.clone()
            });

            self.previously_given_results.insert(chosen_outline, result);
        }

        actor_ref.set_role_state(game, self);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Auditor, 0))
            .available_selection(AvailableTwoRoleOutlineOptionSelection(
                RoleOutlineReference::all_outlines(game)
                    .filter(|o|!self.previously_given_results.contains(o))
                    .map(Some)
                    .chain(once(None))
                    .collect()
            ))
            .night_typical(actor_ref)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Auditor, 0),
            false
        )
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave: GraveReference) {
        let grave = grave.deref(game).clone();
        let GraveInformation::Normal{role, death_cause, ..} = grave.information else {return;};
        let mut grave_roles = self.grave_roles.clone();
            grave_roles.push(role);
            if let GraveDeathCause::Killers(killers) = death_cause {
                for killer in killers {
                    if let GraveKiller::Role(role) = killer {
                        grave_roles.push(role);
                    }
                }
            }
            actor_ref.set_role_state(game, RoleState::Auditor(Auditor{
                previously_given_results: self.previously_given_results, 
                grave_roles
            }));
    }
}

impl Auditor{
    //panics if chosen_outline is not found
    pub fn get_result(game: &Game, chosen_outline: RoleOutlineReference) -> AuditorResult {
        let (role, _) = chosen_outline.deref_as_role_and_player_originally_generated(game);
        
        let role_assignments = chosen_outline.deref(game).get_enabled_role_assignments(game);
        if role_assignments.len() <= 2 {
            AuditorResult::One{role}
        } else {
            let fake_role = role_assignments
                .into_iter()
                .filter(|x|x.role != role)
                .collect::<Vec<RoleAssignment>>()
                .choose(&mut rand::rng())
                .map(|a|a.role);

            if let Some(fake_role) = fake_role{
                let mut two = [role, fake_role];
                two.shuffle(&mut rand::rng());
                AuditorResult::Two{roles: [two[0], two[1]]}
            } else {
                AuditorResult::One{role}
            }
        }
    }
    //panics if chosen_outline is not found
    pub fn get_confused_result(game: &Game, chosen_outline: RoleOutlineReference, grave_roles: &Vec<Role>) -> AuditorResult {        
        let role_assignments = chosen_outline.deref(game).get_enabled_role_assignments(game);
        if role_assignments.len() == 1 {
            return AuditorResult::One {role: role_assignments.first().expect("already checked there is 1").role}
        }
        let is_single = role_assignments.len() <= 2;
        let mut fake_roles: WeightMap<Role> = role_assignments
                .into_iter()
                .map(|assignment| assignment.role)
                .collect::<Vec<Role>>()
                .into();
        for player in PlayerReference::all_players(game) {
            if let Some(fake_role) = Self::get_fib_role(game, player) {
                fake_roles.add_no_insert(fake_role, 1);
            }
        }
            
        for role in grave_roles {
            fake_roles.add_no_insert(*role,1);
        }
        
        if is_single {  
            let role = fake_roles.choose();

            AuditorResult::One {role: role.expect("would imply that the outline could not generate any roles")}
        } else {
            let roles = fake_roles.choose_multiple_remove(2);
            //unsafe because of otherwise having 4 expect statements with reason would obscure the actual code.
            AuditorResult::Two{
                roles: unsafe {[
                    roles.get_unchecked(0).expect("would imply that the length is 0"),
                    roles.get_unchecked(1).expect("would imply that the length is <=1")
                ]}
            }
        }
    }

    /// Returns the role the player is trying/tried to make themselves or someone else look like
    fn get_fib_role(game: &Game, player: PlayerReference) -> Option<Role> {
        match player.role_state(game) {
            RoleState::Disguiser(disguiser) => {
                let fake_role = disguiser.disguised_role(game, player);
                if fake_role == Role::Disguiser && 
                    disguiser.current_target.is_some_and(|p|p==player) {
                    return None;
                }
                Some(fake_role)
            },
            RoleState::Impostor(..) => {
                Impostor::selected_fib_role(game, player)
            },
            RoleState::Counterfeiter(..) => {
                Counterfeiter::selected_forge_role(game, player)
            },
            RoleState::Forger(..) => {
                Forger::selected_forge_role(game, player)
            },
            RoleState::Yer(yer) => { 
                let fake_role = yer.selected_fib_role(game, player);
                
                if fake_role != Role::Yer {
                    return Some(fake_role);
                }
                None
            },
            _=> None,
        }
    }
}