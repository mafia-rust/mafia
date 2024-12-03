use std::iter::once;

use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::components::detained::Detained;
use crate::game::role_outline_reference::RoleOutlineReference;
use crate::game::ability_input::ControllerID;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_map::VecMap;

use rand::prelude::SliceRandom;
use super::{common_role, AbilitySelection, ControllerParametersMap, SavedControllers, AvailableAbilitySelection, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Auditor{
    pub previously_given_results: VecMap<RoleOutlineReference, AuditorResult>,
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
        
        let Some(selection) = SavedControllers::get_two_role_outline_option_selection_if_id(
            game,
            actor_ref,
            ControllerID::role(Role::Auditor, 0)
        )
        else{return};

        if let Some(chosen_outline) = selection.0{
            let result = if Confused::is_confused(game, actor_ref){
                Self::get_confused_result(game, chosen_outline)
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
                Self::get_confused_result(game, chosen_outline)
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
    fn available_abilities(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            actor_ref,
            ControllerID::role(Role::Auditor, 0),
            AvailableAbilitySelection::new_two_role_outline_option(
                RoleOutlineReference::all_outlines(game)
                    .filter(|o|!self.previously_given_results.contains(o))
                    .map(|o|Some(o))
                    .chain(once(None))
                    .collect()
            ),
            AbilitySelection::new_two_role_outline_option(None, None),
            !actor_ref.alive(game) || 
            // game.current_phase().phase() != PhaseType::Night ||
            Detained::is_detained(game, actor_ref),
            Some(PhaseType::Obituary),
            false
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        common_role::convert_saved_ability_to_visits(game, actor_ref, ControllerID::Role { role: Role::Auditor, id: 0 }, false)
    }
}

impl Auditor{
    //panics if chosen_outline is not found
    pub fn get_result(game: &Game, chosen_outline: RoleOutlineReference) -> AuditorResult {
        let (role, _) = chosen_outline.deref_as_role_and_player_originally_generated(game);
        
        let outline = chosen_outline.deref(game);

        if outline.get_roles().len() == 1 || outline.get_roles().len() == 2 {
            AuditorResult::One{role}
        }else{
            let fake_role = outline
                .get_roles()
                .into_iter()
                .filter(|x|game.settings.enabled_roles.contains(x))
                .filter(|x|*x != role)
                .collect::<Vec<Role>>()
                .choose(&mut rand::thread_rng())
                .cloned();

            if let Some(fake_role) = fake_role{
                let mut two = [role, fake_role];
                two.shuffle(&mut rand::thread_rng());
                AuditorResult::Two{roles: [two[0], two[1]]}
            } else {
                AuditorResult::One{role}
            }
        }
    }
    //panics if chosen_outline is not found
    pub fn get_confused_result(game: &Game, chosen_outline: RoleOutlineReference) -> AuditorResult {        
        let outline = chosen_outline.deref(game);

        if outline.get_roles().len() == 1 || outline.get_roles().len() == 2 {
            let fake_role = outline
                .get_roles()
                .into_iter()
                .filter(|x|game.settings.enabled_roles.contains(x))
                .collect::<Vec<Role>>()
                .choose(&mut rand::thread_rng())
                .cloned();

            if let Some(fake_role) = fake_role{
                AuditorResult::One{role: fake_role}
            }else{
                unreachable!("Auditor role outline is empty")
            }
        }else{
            let mut fake_roles = outline
                .get_roles()
                .into_iter()
                .filter(|x|game.settings.enabled_roles.contains(x))
                .collect::<Vec<Role>>();
            
            fake_roles.shuffle(&mut rand::thread_rng());

            let fake_roles = fake_roles.choose_multiple(&mut rand::thread_rng(), 2).cloned().collect::<Vec<Role>>();

            match (fake_roles.get(0), fake_roles.get(1)){
                (Some(role1), Some(role2)) => {
                    AuditorResult::Two{roles: [*role1, *role2]}
                },
                _ => unreachable!("Auditor role outline is empty")
            }
        }
    }
}