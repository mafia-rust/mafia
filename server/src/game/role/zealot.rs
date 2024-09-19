use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::cult::{Cult, CultAbility};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Zealot;

pub type ClientRoleState = Zealot;

pub(super) const FACTION: Faction = Faction::Cult;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for Zealot {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill || Cult::next_ability(game) != CultAbility::Kill {return}

        let Some(visit) = actor_ref.night_visits(game).first() else {return};
        let target_ref = visit.target;
        
        if target_ref.try_night_kill(
            actor_ref, game, GraveKiller::Faction(Faction::Cult), AttackPower::Basic, false
        ) {
            Cult::set_ability_used_last_night(game, Some(CultAbility::Kill));
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        if Cult::next_ability(game) != CultAbility::Kill {return false}

        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
}
