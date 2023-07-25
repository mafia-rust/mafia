use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::{PlayerReference, PlayerIndex};
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Psychic;

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownInvestigative;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Psychic {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
        if actor_ref.night_roleblocked(game) {return;}

        if priority != Priority::Investigative {return}

        actor_ref.push_night_message(game, 
            match game.day_number() % 2 {
                1=>{
                    let all_townies: Vec<_> = PlayerReference::all_players(game)
                        .into_iter()
                        .filter(|p|p.alive(game)&&p.night_appeared_role(game).faction_alignment().faction()==Faction::Town)
                        .collect();

                    if all_townies.len() >= 2 {
                        let mut players = [PlayerIndex, 2];
                        ChatMessage::PsychicGood { players: all_townies.choose_multiple(&mut rand::thread_rng(), 2).collect() }
                    }else{
                        ChatMessage::PsychicFailed
                    }
                },
                _=>{
                    let all_non_townies: Vec<_> = PlayerReference::all_players(game)
                        .into_iter()
                        .filter(|p|p.alive(game)&&p.night_appeared_role(game).faction_alignment().faction()!=Faction::Town)
                        .collect();

                    if all_townies.len() >= 3 {
                        all_townies.choose_multiple(rand::random(), 2)
                        ChatMessage::PsychicEvil { players: () }
                    }else{
                        ChatMessage::PsychicFailed
                    }
                },
            }
        );
        
        
        
        

    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        // crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}