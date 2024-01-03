use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, Role};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Arsonist;

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Arsonist {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Arsonist}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        

        match priority {
            Priority::Deception => {
                if !actor_ref.night_jailed(game) {
                    //douse target
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;
                        if target_ref.night_jailed(game){
                            actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                            return
                        }
                        if Role::Arsonist != target_ref.role_state(game).role() {
                            target_ref.set_doused(game, true);
                        }
                    }

                    //douse all visitors
                    for other_player_ref in PlayerReference::all_players(game)
                        .into_iter()
                        .filter(|other_player_ref|
                            *other_player_ref != actor_ref &&
                            other_player_ref.night_visits(game)
                                .iter()
                                .any(|v|!v.astral&&v.target==actor_ref)
                        ).collect::<Vec<PlayerReference>>()
                    {
                        if Role::Arsonist != other_player_ref.role_state(game).role() {
                            other_player_ref.set_doused(game, true);
                        }
                    }
                }
                


                //Make all doused players look like arsonist
                for player_ref in PlayerReference::all_players(game){
                    if player_ref.doused(game) {
                        player_ref.set_night_appeared_role(game, Role::Arsonist);
                    }
                }

                //undouse me
                actor_ref.set_doused(game, false);
            },
            Priority::Kill => {
                if actor_ref.night_jailed(game) {return}
                
                if let Some(visit) = actor_ref.night_visits(game).first(){
                    if actor_ref == visit.target{
                        for player_ref in PlayerReference::all_players(game){
                            if player_ref.doused(game) {
                                player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Arsonist), 3, true);
                            }
                        }
                    }
                }
                
            },
            Priority::Investigative => {
                //show player tags on doused players
                actor_ref.remove_player_tag_on_all(game, Tag::Doused);
                for player_ref in PlayerReference::all_players(game){
                    if player_ref.doused(game) {
                        actor_ref.push_player_tag(game, player_ref, Tag::Doused)
                    }
                }
            },
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
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
