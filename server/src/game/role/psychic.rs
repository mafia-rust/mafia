use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::visit::Visit;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;


use crate::game::Game;
use super::{Priority, RoleStateImpl};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Psychic;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Psychic {
    type ClientRoleState = Psychic;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return}

        
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};

        actor_ref.push_night_message(game, 
            if game.day_number() % 2 == 1 {
                Psychic::get_result_evil(game, actor_ref, visit.target, Confused::is_confused(game, actor_ref))
            }else{
                Psychic::get_result_good(game, actor_ref, visit.target, Confused::is_confused(game, actor_ref))
            }
        );
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
}

impl Psychic {
    fn get_result_evil(game: &Game, actor_ref: PlayerReference, target: PlayerReference, confused: bool)->ChatMessageVariant{
        
        let mut valid_players: Vec<_> = Self::get_valid_players(game, actor_ref, target)
            .into_iter()
            .filter(|p|!p.has_innocent_aura(game))
            .collect();

        valid_players.shuffle(&mut rand::thread_rng());

        for i in 0..valid_players.len(){
            for j in i+1..valid_players.len(){
                let out = [target, valid_players[i], valid_players[j]];

                if confused || Self::contains_evil(game, out){
                    return ChatMessageVariant::PsychicEvil { players: [out[0].index(), out[1].index(), out[2].index()] }
                }
            }
        }

        ChatMessageVariant::PsychicFailed
    }
    fn get_result_good(game: &Game, actor_ref: PlayerReference, target: PlayerReference, confused: bool)->ChatMessageVariant{
        let mut valid_players: Vec<_> = Self::get_valid_players(game, actor_ref, target)
            .into_iter()
            .filter(|p|!p.has_suspicious_aura(game))
            .collect();

        valid_players.shuffle(&mut rand::thread_rng());

        for i in 0..valid_players.len(){
            let out = [target, valid_players[i]];

            if confused || Self::contains_good(game, out){
                return ChatMessageVariant::PsychicGood { players: [out[0].index(), out[1].index()] }
            }
        }

        ChatMessageVariant::PsychicFailed
    }

    fn player_is_evil(game: &Game, player_ref: PlayerReference)-> bool {
        !player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town)
    }
    fn get_valid_players(game: &Game, actor_ref: PlayerReference, target: PlayerReference)->Vec<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|*p != actor_ref)
            .filter(|p|*p != target)
            .filter(|p|p.alive(game))
            .collect()
    }

    fn contains_evil(game: &Game, player_refs: [PlayerReference; 3])->bool{
        player_refs.into_iter().any(|player_ref|Psychic::player_is_evil(game, player_ref))
    }
    fn contains_good(game: &Game, player_refs: [PlayerReference; 2])->bool{
        player_refs.into_iter().any(|player_ref|!Psychic::player_is_evil(game, player_ref))
    }
}