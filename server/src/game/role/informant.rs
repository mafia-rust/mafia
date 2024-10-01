use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Informant{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Informant {
    type ClientRoleState = Informant;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;

            let mut visited_by: Vec<PlayerReference> =  visit.target.appeared_visitors(game).into_iter().filter(|p|actor_ref!=*p).collect();
            visited_by.shuffle(&mut thread_rng());

            let mut visited: Vec<PlayerReference> = target_ref.tracker_seen_visits(game).iter().map(|v|v.target).collect();
            visited.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::InformantResult{
                role: target_ref.role(game), 
                visited_by: PlayerReference::ref_vec_to_index(&visited_by.as_mut_slice()),
                visited: PlayerReference::ref_vec_to_index(visited.as_slice())
            };
            actor_ref.push_night_message(game, message);
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, &action_choice, false){
            return
        }
        if game.day_number() == 1 {return}

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