use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::common_role::default_action_choice_two_players_is_valid;
use super::{Priority, RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Transporter{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Transporter {
    type ClientRoleState = Transporter;
    type RoleActionChoice = super::common_role::RoleActionChoiceTwoPlayers;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Transporter {return;}
    
        let transporter_visits = actor_ref.night_visits(game).clone();
        let Some(first_visit) = transporter_visits.get(0) else {return};
        let Some(second_visit) = transporter_visits.get(1) else {return};
        
        
        first_visit.target.push_night_message(game, ChatMessageVariant::Transported);
        second_visit.target.push_night_message(game, ChatMessageVariant::Transported);
    
        for player_ref in PlayerReference::all_players(game){
            if player_ref == actor_ref {continue;}
            if player_ref.role(game) == Role::Transporter {continue;}

            let new_visits = player_ref.night_visits(game).clone().into_iter().map(|mut v|{
                if v.target == first_visit.target {
                    v.target = second_visit.target;
                } else if v.target == second_visit.target{
                    v.target = first_visit.target;
                }
                v
            }).collect();
            player_ref.set_night_visits(game, new_visits);
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        let Some(..) = action_choice.two_players else {
            self = Transporter{ night_selection: action_choice };
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