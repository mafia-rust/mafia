use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, PlayerListSelection, Priority, Role, RoleStateImpl};


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
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Godfather, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Godfather, 0),
            true
        )
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        let Some(PlayerListSelection(backup)) = game.saved_controllers
            .get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        )
        else {return};
        let Some(backup) = backup.first() else {return};
        if actor_ref != dead_player_ref {return}

        for player_ref in PlayerReference::all_players(game){
            if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref) {
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        //convert backup to godfather
        backup.set_role_and_win_condition_and_revealed_group(game, Godfather);
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
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
            _ => return
        }
    }
}