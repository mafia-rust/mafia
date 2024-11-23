use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleStateImpl};

#[derive(Debug, Clone, Serialize, Default)]
pub struct SerialKiller;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for SerialKiller {
    type ClientRoleState = SerialKiller;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        if game.day_number() == 1 {return}


        let actor_visits = actor_ref.night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){

            let target_ref = visit.target;
            
            target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::SerialKiller), AttackPower::Basic, true);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && game.day_number() > 1
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
}
