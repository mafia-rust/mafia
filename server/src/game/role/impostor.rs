use serde::Serialize;

use crate::game::ability_input::*;
use crate::game::attack_power::DefensePower;
use crate::game::grave::GraveInformation;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::godfather::Godfather;
use super::{Priority, Role, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Impostor{
    pub backup: Option<PlayerReference>
}

impl Default for Impostor {
    fn default() -> Self {
        Self {
            backup: None,
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Impostor {
    type ClientRoleState = Impostor;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        Godfather::night_ability(game, actor_ref, priority);
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Impostor, 0),
            true
        )
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Impostor, 0)
        ).combine_overwrite_owned(ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Impostor, 1),
            AvailableAbilitySelection::new_role_option(
                Role::values().into_iter()
                    .map(|role| Some(role))
                    .collect()
            ),
            AbilitySelection::new_role_option(Some(Role::Impostor)),
            !actor_ref.alive(game),
            None,
            false,
            vec_set!(actor_ref)
        ))
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave: crate::game::grave::GraveReference) {
        let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
            ControllerID::role(actor_ref, Role::Impostor, 1)
        )else{return};
        
        
        if grave.deref(game).player == actor_ref {
            let grave = grave.deref_mut(game);
            grave.information = match grave.information.clone() {
                GraveInformation::Obscured => GraveInformation::Obscured,
                GraveInformation::Normal {will, death_cause, death_notes, .. } => {
                    GraveInformation::Normal { role, will, death_cause, death_notes } 
                },
            };
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        let Some(PlayerListSelection(backup)) = game.saved_controllers
            .get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        )
        else {return};
        let Some(backup) = backup.first() else {return};
        if actor_ref != dead_player_ref {return}

        //convert backup to godfather
        backup.set_role_and_win_condition_and_revealed_group(game, Godfather);
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}