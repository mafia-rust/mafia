use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::components::confused::Confused;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{chat::ChatMessageVariant, components::verdicts_today::VerdictsToday};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::RoleStateImpl;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct TallyClerk;


impl RoleStateImpl for TallyClerk {
    type ClientRoleState = TallyClerk;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if actor_ref.night_blocked(midnight_variables) {return}
        if actor_ref.ability_deactivated_from_death(game) {return}
        if priority != OnMidnightPriority::Investigative {return;}

        let mut evil_count: u8 = 0;
        for player in PlayerReference::all_players(game)
            .filter(|player|player.alive(game))
            .filter(|player|VerdictsToday::player_guiltied_today(game, player))
        {
            if TallyClerk::player_is_suspicious(game, midnight_variables, player){
                evil_count = evil_count.saturating_add(1);
            }
        }

        if Confused::is_confused(game, actor_ref){
            let total_guilties = VerdictsToday::guilties(game).len();
            //add or subtract 1 randomly from the count
            if rand::random::<bool>(){
                evil_count = (evil_count.saturating_add(1u8)).min(total_guilties.try_into().unwrap_or(u8::MAX));
            }else{
                evil_count = evil_count.saturating_sub(1u8);
            }
        }

        
        let message = ChatMessageVariant::TallyClerkResult{ evil_count };
        actor_ref.push_night_message(midnight_variables, message);
    }
}

impl TallyClerk {
    pub fn player_is_suspicious(game: &Game, midnight_variables: &MidnightVariables, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game, midnight_variables){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town)
        }
    }
}