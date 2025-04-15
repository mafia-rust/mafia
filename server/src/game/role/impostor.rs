use serde::Serialize;

use crate::game::ability_input::*;
use crate::game::attack_power::DefensePower;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::grave::GraveInformation;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::godfather::Godfather;
use super::{Role, RoleStateImpl};


#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Impostor{
    pub backup: Option<PlayerReference>
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Impostor {
    type ClientRoleState = Impostor;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        Godfather::night_kill_ability(game, actor_ref, priority);
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Impostor, 0),
            true
        )
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Impostor, 0))
                .single_player_selection_typical(actor_ref, false, false)
                .night_typical(actor_ref)
                .add_grayed_out_condition(game.day_number() <= 1)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Impostor, 1))
                .available_selection(AvailableRoleOptionSelection(
                    Role::values().into_iter()
                        .map(Some)
                        .collect()
                ))
                .default_selection(RoleOptionSelection(Some(Role::Impostor)))
                .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
                .allow_players([actor_ref])
                .build_map()
        ])
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
        Godfather::pass_role_state_down(game, actor_ref, dead_player_ref, self);
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}