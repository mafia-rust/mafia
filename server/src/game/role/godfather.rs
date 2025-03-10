use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::attack_type::AttackData;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, PlayerListSelection, Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Godfather;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Godfather {
    type ClientRoleState = Godfather;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        Self::night_ability(game, actor_ref, priority);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Godfather, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Godfather, 0),
            true
        )
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        Self::pass_role_state_down(game, actor_ref, dead_player_ref, self);
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

impl Godfather{
    pub(super) fn night_ability(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() == 1 {return}

        match priority {
            //kill the target
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                visit.target.clone().try_night_kill_single_attacker(
                    actor_ref, game, GraveKiller::RoleSet(RoleSet::Mafia),
                    AttackPower::Basic, false
                );
            },
            _ => {}
        }
    }
    pub (super) fn pass_role_state_down(
        game: &mut Game,
        actor_ref: PlayerReference,
        dead_player_ref: PlayerReference,
        new_role_data: impl Into<RoleState>
    ){
        let Some(PlayerListSelection(backup)) = game.saved_controllers
            .get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        )else {return};
        let Some(backup) = backup.first() else {return};
        if actor_ref != dead_player_ref {return}

        //convert backup to godfather
        backup.set_role(game, new_role_data);
    }
}