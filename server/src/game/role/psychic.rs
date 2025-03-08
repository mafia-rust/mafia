use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::visit::Visit;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;


use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

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
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Psychic, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Psychic, 0),
            false
        )
    }
}

impl Psychic {
    fn get_result_evil(game: &Game, actor_ref: PlayerReference, target: PlayerReference, confused: bool)->ChatMessageVariant{
        
        let mut valid_players: Vec<_> = Self::get_valid_players(game, actor_ref, target)
            .into_iter()
            .filter(|p|!p.has_innocent_aura(game))
            .collect();

        valid_players.shuffle(&mut rand::rng());

        #[expect(clippy::indexing_slicing)]
        for i in 0..valid_players.len(){
            #[expect(clippy::arithmetic_side_effects)]
            for j in i+1..valid_players.len(){
                if confused || Self::contains_evil(game, target, valid_players[i], valid_players[j]){
                    return ChatMessageVariant::PsychicEvil { first: valid_players[i], second: valid_players[j] }
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

        valid_players.shuffle(&mut rand::rng());

        for player in valid_players{
            if confused || Self::contains_good(game, target, player){
                return ChatMessageVariant::PsychicGood { player }
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

    fn contains_evil(game: &Game, target: PlayerReference, a: PlayerReference, b: PlayerReference)->bool{
        [target, a, b].into_iter().any(|player_ref|Psychic::player_is_evil(game, player_ref))
    }
    fn contains_good(game: &Game, target: PlayerReference, player: PlayerReference)->bool{
        [target, player].into_iter().any(|player_ref|!Psychic::player_is_evil(game, player_ref))
    }
}