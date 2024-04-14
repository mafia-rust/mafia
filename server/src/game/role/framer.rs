use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{same_evil_team, Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Framer {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Deception {return;}
    
        let framer_visits = actor_ref.night_visits(game).clone();


        let Some(first_visit) = framer_visits.first() else {return};

        if first_visit.target.night_jailed(game) {
            actor_ref.push_night_message(game, ChatMessageVariant::TargetJailed);
        }else{
            first_visit.target.set_night_framed(game, true);
        }
        


        let Some(second_visit) = framer_visits.get(1) else {return};
    
        if !first_visit.target.night_jailed(game) {
            first_visit.target.set_night_appeared_visits(game, Some(vec![
                Visit{ target: second_visit.target, attack: false }
            ]));
        }

        actor_ref.set_night_visits(game, vec![first_visit.clone()]);
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        (
            actor_ref.chosen_targets(game).is_empty() &&
            actor_ref != target_ref &&
            target_ref.alive(game) &&
            !same_evil_team(game, actor_ref, target_ref)
        ) || 
        (
            actor_ref.chosen_targets(game).len() == 1
        )
        
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], attack: false }, 
                Visit{ target: target_refs[1], attack: false }
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit{ target: target_refs[0], attack: false }
            ]
        } else {
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
