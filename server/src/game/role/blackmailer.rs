use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::PlayerReference};
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Blackmailer{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Blackmailer {
    type ClientRoleState = Self;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Deception {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
    
            target_ref.set_night_silenced(game, true);
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false){
            return
        }

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
    }
}
