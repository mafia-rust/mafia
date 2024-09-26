use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::PlayerReference};
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Blackmailer;

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Blackmailer {
    type ClientRoleState = Blackmailer;
    type RoleActionChoice = super::common_role::CommonRoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Deception {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
    
            target_ref.set_night_silenced(game, true);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
}
