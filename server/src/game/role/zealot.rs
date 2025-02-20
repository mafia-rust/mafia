use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::cult::{Cult, CultAbility};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;


use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Zealot;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Zealot {
    type ClientRoleState = Zealot;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill || Cult::next_ability(game) != CultAbility::Kill {return}

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(visit) = actor_visits.first() else {return};
        let target_ref = visit.target;
        
        if target_ref.try_night_kill_single_attacker(
            actor_ref, game, GraveKiller::RoleSet(RoleSet::Cult), AttackPower::Basic, false
        ) {
            Cult::set_ability_used_last_night(game, Some(CultAbility::Kill));
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            Cult::next_ability(game) != CultAbility::Kill,
            ControllerID::role(actor_ref, Role::Zealot, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Zealot, 0),
            true
        )
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Cult
        ].into_iter().collect()
    }
}
