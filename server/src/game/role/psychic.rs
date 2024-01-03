use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
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
        if !actor_ref.alive(game) {return;}

        if priority != Priority::Investigative {return}

        let all_players: Vec<_> = PlayerReference::all_players(game)
            .into_iter()
            .filter(|p|p.alive(game)&&*p!=actor_ref)
            .collect();

        let mut rng = rand::thread_rng();
        actor_ref.push_night_message(game, 'a: {match game.day_number() % 2 {
           0=>{
                let all_non_townies: Vec<_> = PlayerReference::all_players(game)
                    .into_iter()
                    .filter(|p|p.alive(game)&&*p!=actor_ref&&p.night_appeared_role(game).faction_alignment().faction()!=Faction::Town)
                    .collect();

                if let Some(non_townie) = all_non_townies.choose(&mut rng){
                    let random_players: Vec<PlayerReference> = all_players.into_iter()
                        .filter(|p|p!=non_townie)
                        .collect::<Vec<_>>()
                        .choose_multiple(&mut rng, 2)
                        .copied()
                        .collect();
                    
                    if let Some(random_player0) = random_players.get(0){
                        if let Some(random_player1) = random_players.get(1){

                            let mut out = [non_townie, random_player0, random_player1];
                            out.shuffle(&mut rng);
                            break 'a ChatMessage::PsychicEvil { players: [out[0].index(), out[1].index(), out[2].index()] }
                        }
                    }
                }
                ChatMessage::PsychicFailed
            },
            _=>{
                let all_townies: Vec<_> = PlayerReference::all_players(game)
                    .into_iter()
                    .filter(|p|p.alive(game)&&*p!=actor_ref&&p.night_appeared_role(game).faction_alignment().faction()==Faction::Town)
                    .collect();

                if let Some(townie) = all_townies.choose(&mut rng){
                    if let Some(random_player) = all_players.into_iter()
                        .filter(|p|p!=townie)
                        .collect::<Vec<_>>()
                        .choose(&mut rand::thread_rng()){
                        
                        let mut out = [townie, random_player];
                        out.shuffle(&mut rng);

                        break 'a ChatMessage::PsychicGood { players: [out[0].index(), out[1].index()] }   
                    }
                }
                ChatMessage::PsychicFailed
            },
        }});
        
        
        
        

    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
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