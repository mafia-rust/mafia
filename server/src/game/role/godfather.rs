use serde::Serialize;

use crate::game::ability_input::ControllerParametersMap;
use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, PlayerListSelection, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Godfather;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Godfather {
    type ClientRoleState = Godfather;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        Self::night_ability(game, actor_ref, priority);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Godfather, 0))
            .single_player_selection_typical(actor_ref, false, false)
            .night_typical(actor_ref)
            .add_grayed_out_condition(game.day_number() <= 1)
            .build_map()
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
}

impl Godfather{
    pub(super) fn night_ability(game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if game.day_number() == 1 {return}

        match priority {
            //kill the target
            OnMidnightPriority::Kill => {
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
        if actor_ref != dead_player_ref {return}
        let Some(PlayerListSelection(backup)) = game.saved_controllers
            .get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        )else {return};
        
        let Some(backup) = backup.first() else {return};
        
        //convert backup to godfather
        backup.set_role(game, new_role_data);
    }
}