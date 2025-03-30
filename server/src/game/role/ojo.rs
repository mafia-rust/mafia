use serde::Serialize;

use crate::game::ability_input::{AvailableRoleOptionSelection, AvailableTwoRoleOutlineOptionSelection, ControllerID};
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::role_outline_reference::RoleOutlineReference;
use crate::game::visit::Visit;
use crate::game::{attack_power::AttackPower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::Game;
use crate::vec_map::VecMap;
use super::auditor::AuditorResult;
use super::{common_role, ControllerParametersMap, Priority, Role, RoleOptionSelection, RoleStateImpl, TwoRoleOutlineOptionSelection};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ojo{
    pub previously_given_results: VecMap<RoleOutlineReference, AuditorResult>,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Ojo {
    type ClientRoleState = Ojo;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if actor_ref.night_blocked(game) {return;}

        match priority {
            Priority::Kill => {
                if game.day_number() == 1 {return;}
                for visit in actor_ref.untagged_night_visits_cloned(game).clone() {
                    if visit.attack {
                        visit.target.try_night_kill_single_attacker(
                            actor_ref, game, 
                            GraveKiller::Role(Role::Ojo), 
                            AttackPower::ArmorPiercing, 
                            true
                        );
                    }
                }
            },
            Priority::Investigative => {
                let visited_me = actor_ref.all_night_visitors_cloned(game);

                for player in PlayerReference::all_players(game) {
                    if visited_me.contains(&player) {
                        actor_ref.insert_role_label(game, player);
                    }
                }

                if let Some(TwoRoleOutlineOptionSelection(a, b)) = game.saved_controllers.get_controller_current_selection_two_role_outline_option(
                    ControllerID::role(actor_ref, Role::Ojo, 0)
                ){
                    if let Some(chosen_outline) = a{
                        let result = Self::get_result(game, chosen_outline);
                        actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                            role_outline: chosen_outline.deref(game).clone(),
                            result: result.clone()
                        });
                        self.previously_given_results.insert(chosen_outline, result);
                    }
            
                    if let Some(chosen_outline) = b{
                        let result = Self::get_result(game, chosen_outline);
                        actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                            role_outline: chosen_outline.deref(game).clone(),
                            result: result.clone()
                        });
                        self.previously_given_results.insert(chosen_outline, result);
                    }
                }
        
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
    
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Ojo, 0))
                .available_selection(AvailableTwoRoleOutlineOptionSelection(
                    RoleOutlineReference::all_outlines(game)
                        .filter(|o|!self.previously_given_results.contains(o))
                        .map(Some)
                        .chain(std::iter::once(None))
                        .collect()
                ))
                .night_typical(actor_ref)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Ojo, 1))
                .available_selection(AvailableRoleOptionSelection(
                    Role::values().into_iter().map(Some).chain(std::iter::once(None)).collect()
                ))
                .night_typical(actor_ref)
                .add_grayed_out_condition(game.day_number() == 1)
                .build_map()
        ])
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let mut out = vec![];

        out.extend(common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Ojo, 0),
            false
        ));

        if game.day_number() > 1 {
            if let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
                ControllerID::role(actor_ref, Role::Ojo, 1)
            ) {
                for player in PlayerReference::all_players(game){
                    if player.alive(game) && player.role(game) == role {
                        out.push(Visit::new_none(actor_ref, player, true));
                    }
                }
            }
        }

        out
    }
}

impl Ojo{
    //panics if chosen_outline is not found
    pub fn get_result(game: &Game, chosen_outline: RoleOutlineReference) -> AuditorResult {
        let (role, _) = chosen_outline.deref_as_role_and_player_originally_generated(game);
        AuditorResult::One{role}
    }
}