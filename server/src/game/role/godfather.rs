use serde::Serialize;

use crate::game::ability_input::ControllerParametersMap;
use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, PlayerListSelection, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Godfather;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armored;

impl RoleStateImpl for Godfather {
    type ClientRoleState = Godfather;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        Self::night_kill_ability(game, midnight_variables, actor_ref, priority);

        if priority == OnMidnightPriority::Deception{
            let Some(PlayerListSelection(players)) = ControllerID::role(actor_ref, Role::Godfather, 1).get_player_list_selection(game) else {return};
            let Some(appeared_into) = players.first() else {return};
            actor_ref.set_night_appeared_visits(midnight_variables, Some(vec![
                Visit::new_none(actor_ref, *appeared_into, false)
            ]));
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Godfather, 0))
                .single_player_selection_typical(actor_ref, false, false)
                .night_typical(actor_ref)
                .add_grayed_out_condition(game.day_number() <= 1)
                .build_map(),

            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Godfather, 1))
                .single_player_selection_typical(actor_ref, true, true)
                .night_typical(actor_ref)
                .build_map()
        ])
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
    pub(super) fn night_kill_ability(game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if game.day_number() == 1 {return}

        match priority {
            //kill the target
            OnMidnightPriority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                visit.target.clone().try_night_kill_single_attacker(
                    actor_ref, game, midnight_variables, GraveKiller::RoleSet(RoleSet::Mafia),
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
        let Some(PlayerListSelection(backup)) = ControllerID::syndicate_choose_backup().get_player_list_selection(game) else {return};
        let Some(backup) = backup.first().copied() else {return};

        //convert backup to godfather
        backup.set_role(game, new_role_data);
    }
}