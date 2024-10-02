use serde::Serialize;

use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::common_role::default_action_choice_two_players_is_valid;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Cupid{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cupid {
    type ClientRoleState = Self;
    type RoleActionChoice = super::common_role::RoleActionChoiceTwoPlayers;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Cupid => {
                let visits = actor_ref.night_visits(game);

                let Some(first_visit) = visits.get(0) else {return};
                let Some(second_visit) = visits.get(1) else {return};
                
                let player1 = first_visit.target;
                let player2 = second_visit.target;

                LoveLinked::add_love_link(game, player1, player2);
            },
            _ => ()
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        let Some(..) = action_choice.two_players else {
            self = Cupid{ night_selection: action_choice };
            actor_ref.set_role_state(game, self);
            return
        };

        if !default_action_choice_two_players_is_valid(game, actor_ref, &action_choice, false) {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);    
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits_two_players(&self.night_selection, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
    }
}
