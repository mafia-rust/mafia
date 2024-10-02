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
pub struct Zealot{
    pub night_selection: super::common_role::RoleActionChoiceOnePlayer
}

pub(super) const FACTION: Faction = Faction::Cult;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Zealot {
    type ClientRoleState = Zealot;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill || Cult::next_ability(game) != CultAbility::Kill {return}

        let Some(visit) = actor_ref.night_visits(game).first() else {return};
        let target_ref = visit.target;
        
        if target_ref.try_night_kill_single_attacker(
            actor_ref, game, GraveKiller::Faction(Faction::Cult), AttackPower::Basic, false
        ) {
            Cult::set_ability_used_last_night(game, Some(CultAbility::Kill));
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false){
            return
        }

        if Cult::next_ability(game) != CultAbility::Kill {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection, true)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
    }
}
