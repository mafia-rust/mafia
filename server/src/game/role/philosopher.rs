use serde::Serialize;

use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Philosopher{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Philosopher {
    type ClientRoleState = Philosopher;
    type RoleActionChoice = super::common_role::RoleActionChoiceTwoPlayers;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        let Some(first_visit) = actor_ref.night_visits(game).get(0) else {return;};
        let Some(second_visit) = actor_ref.night_visits(game).get(1) else {return;};

        let message = ChatMessageVariant::SeerResult{
            enemies: Philosopher::players_are_enemies(game, first_visit.target, second_visit.target)
        };
        
        actor_ref.push_night_message(game, message);
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        let Some(..) = action_choice.two_players else {
            self = Philosopher{ night_selection: action_choice };
            actor_ref.set_role_state(game, self);
            return
        };

        if !super::common_role::default_action_choice_two_players_is_valid(game, actor_ref, action_choice.two_players, (false, false), false) {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);    
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits_two_players(self.night_selection.two_players, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
    }
}
impl Philosopher{
    pub fn players_are_enemies(game: &Game, a: PlayerReference, b: PlayerReference) -> bool {
        if a.has_suspicious_aura(game) || b.has_suspicious_aura(game){
            true
        }else if a.has_innocent_aura(game) || b.has_innocent_aura(game){
            false
        }else{
            !WinCondition::can_win_together(a.win_condition(game), b.win_condition(game))
        }
    }
}