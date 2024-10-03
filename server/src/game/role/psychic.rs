use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::{Priority, RoleStateImpl};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Psychic;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Psychic {
    type ClientRoleState = Psychic;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if actor_ref.night_blocked(game) {return}
        if !actor_ref.alive(game) {return}

        if priority != Priority::Investigative {return}

        actor_ref.push_night_message(game, match game.day_number() % 2 {
            1=>{
                Psychic::get_psychic_result_evil(game, actor_ref)
            },
            _=>{
                Psychic::get_psychic_result_good(game, actor_ref)
            },
        });
        
    }
}

impl Psychic {
    fn get_psychic_result_evil(game: &Game, actor_ref: PlayerReference)->ChatMessageVariant{
        
        let mut rng = rand::thread_rng();

        let evil_players: Vec<_> = Psychic::get_valid_players(game, actor_ref).into_iter()
            .filter(|player_ref|Psychic::player_is_evil(game, *player_ref))
            .filter(|player_ref|!player_ref.has_innocent_aura(game))
            .collect();

        let Some(selected_evil) = evil_players.choose(&mut rng) else {return ChatMessageVariant::PsychicFailed};

        let random_players: Vec<_> = Psychic::get_valid_players(game, actor_ref).into_iter()
            .filter(|p|p!=selected_evil)
            .filter(|player_ref|!player_ref.has_innocent_aura(game))
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, 2).copied().collect();
        
        let Some(random_player0) = random_players.get(0) else {return ChatMessageVariant::PsychicFailed};
        let Some(random_player1) = random_players.get(1) else {return ChatMessageVariant::PsychicFailed};

        let mut out = [selected_evil, random_player0, random_player1];
        out.shuffle(&mut rng);
        ChatMessageVariant::PsychicEvil { players: [out[0].index(), out[1].index(), out[2].index()] }

    }
    fn get_psychic_result_good(game: &Game, actor_ref: PlayerReference)->ChatMessageVariant{
        let mut rng = rand::thread_rng();

        let good_players: Vec<_> = Psychic::get_valid_players(game, actor_ref).into_iter()
            .filter(|player_ref|!Psychic::player_is_evil(game, *player_ref))
            .filter(|player_ref|!player_ref.has_suspicious_aura(game))
            .collect();

        let Some(selected_good) = good_players.choose(&mut rng) else {return ChatMessageVariant::PsychicFailed};

        let random_players: Vec<_> = Psychic::get_valid_players(game, actor_ref).into_iter()
            .filter(|player_ref|!player_ref.has_suspicious_aura(game))
            .filter(|p|p!=selected_good)
            .collect::<Vec<_>>();
        
        let Some(random_player) = random_players.choose(&mut rng) else {return ChatMessageVariant::PsychicFailed};

        let mut out = [selected_good, random_player];
        out.shuffle(&mut rng);
        ChatMessageVariant::PsychicGood { players: [out[0].index(), out[1].index()] }
    }


    fn get_valid_players(game: &Game, actor_ref: PlayerReference)->Vec<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|player_ref|player_ref.alive(game) && *player_ref!=actor_ref)
            .collect()
    }

    fn player_is_evil(game: &Game, player_ref: PlayerReference)-> bool {
        !player_ref.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town)
    }
}