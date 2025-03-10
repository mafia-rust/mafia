use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::attack_type::AttackData;
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{common_role, ControllerID, Priority, Role, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Mafioso;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Mafioso {
    type ClientRoleState = Mafioso;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        if game.day_number() == 1 {return}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;
    
            target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::RoleSet(RoleSet::Mafia), AttackPower::Basic, false);
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Mafioso, 0)
        )  
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Mafioso, 0),
            true
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn attack_data(&self, game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData::attack(game, actor_ref, false, false)
    }
}