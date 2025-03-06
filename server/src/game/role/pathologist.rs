use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;


use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Pathologist;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Pathologist {
    type ClientRoleState = Pathologist;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        if game.day_number() == 1 {return}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;
            target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::RoleSet(RoleSet::Mafia), AttackPower::Basic, false);
        }
    }
}