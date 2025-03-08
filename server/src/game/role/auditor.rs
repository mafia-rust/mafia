use std::iter::once;

use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::components::detained::Detained;
use crate::game::role_outline_reference::RoleOutlineReference;
use crate::game::ability_input::*;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_map::VecMap;
use crate::vec_set::vec_set;
use crate::game::grave::{GraveDeathCause, GraveInformation, GraveKiller, GraveReference};
use crate::weight_map::WeightMap;
use rand::prelude::SliceRandom;
use super::counterfeiter::Counterfeiter;
use super::disguiser::Disguiser;
use super::forger::Forger;
use super::impostor::Impostor;
use super::{common_role, Priority, Role, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Auditor{
    pub previously_given_results: VecMap<RoleOutlineReference, AuditorResult>,
    pub grave_roles: Vec<Role>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
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
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if priority != Priority::Investigative {return;}
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
                role_outline: chosen_outline.deref(&game).clone(),
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
                role_outline: chosen_outline.deref(&game).clone(),
                result: result.clone()
            });

            self.previously_given_results.insert(chosen_outline, result);
        }

        actor_ref.set_role_state(game, self);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Auditor, 0),
            AvailableAbilitySelection::new_two_role_outline_option(
                RoleOutlineReference::all_outlines(game)
                    .filter(|o|!self.previously_given_results.contains(o))
                    .map(|o|Some(o))
                    .chain(once(None))
                    .collect()
            ),
            AbilitySelection::new_two_role_outline_option(None, None),
            actor_ref.ability_deactivated_from_death(game) ||
            Detained::is_detained(game, actor_ref),
            Some(PhaseType::Obituary),
            false,
            vec_set![actor_ref],
        )
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
        
        let outline = chosen_outline.deref(game);

        if outline.get_role_assignments().len() == 1 || outline.get_role_assignments().len() == 2 {
            AuditorResult::One{role}
        }else{
            let fake_role = outline
                .get_role_assignments()
                .into_iter()
                .map(|data| data.role)
                .filter(|x|game.settings.enabled_roles.contains(x))
                .filter(|x|*x != role)
                .collect::<Vec<Role>>()
                .choose(&mut rand::rng())
                .cloned();

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
        let outline = chosen_outline.deref(game);

        if outline.get_role_assignments().len() == 1 || outline.get_role_assignments().len() == 2 {
            let fake_role = outline
                .get_role_assignments()
                .into_iter()
                .map(|assignment| assignment.role)
                .filter(|x|game.settings.enabled_roles.contains(x))
                .collect::<Vec<Role>>()
                .choose(&mut rand::rng())
                .cloned();

            if let Some(fake_role) = fake_role{
                AuditorResult::One{role: fake_role}
            }else{
                unreachable!("Auditor role outline is empty")
            }
        } else {
            let mut fake_roles = WeightMap::from(outline
                .get_role_assignments()
                .into_iter()
                .map(|assignment| assignment.role)
                .filter(|x|game.settings.enabled_roles.contains(x))
                .collect::<Vec<Role>>());
            
            for player in PlayerReference::all_players(game) {
                let Some(fake_role) = Self::fake_role(game, player) else {continue};
                fake_roles.add_no_insert(fake_role, 1);
            }
                
            for role in grave_roles {
                fake_roles.add_no_insert(*role,1);
            }

            let fake_roles = fake_roles.choose_multiple(2);

            match (fake_roles.get(0), fake_roles.get(1)){
                (Some(Some(role1)), Some(Some(role2))) => {
                    AuditorResult::Two{roles: [*role1, *role2]}
                },
                _ => unreachable!("Auditor role outline is empty")
            }
        }
    }

    /// Returns the role the player is trying/tried to make themselves or someone else look like
    fn fake_role(game: &Game, player: PlayerReference) -> Option<Role> {
        match player.role_state(game) {
            RoleState::Disguiser(disguiser) => {
                let fake_role = Disguiser::disguised_role(game, player);
                if fake_role == Role::Disguiser && disguiser.players_with_disguiser_menu(player).contains(&player){
                    return None;
                }
                return Some(fake_role);
            },
            RoleState::Impostor(..) => {
                return Impostor::current_fake_role(game, player);
            },
            RoleState::Counterfeiter(..) => {
                if Counterfeiter::chose_no_forge(game, player) {
                    return None;
                };
                return Counterfeiter::selected_forge_role(game, player);
            },
            RoleState::Forger(..) => {
                return Forger::selected_forge_role(game, player);
            },
            RoleState::Yer(yer) => { 
                let fake_role = yer.current_fake_role(game, player);
                if fake_role != Role::Yer {
                    return Some(fake_role);
                }
                return None;
            },
            _=> return None,
        }
    }
}