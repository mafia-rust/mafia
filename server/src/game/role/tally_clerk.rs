use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::components::confused::Confused;
use crate::game::{chat::ChatMessageVariant, components::verdicts_today::VerdictsToday};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::{Priority, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct TallyClerk;


impl RoleStateImpl for TallyClerk {
    type ClientRoleState = TallyClerk;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_blocked(game) {return}
        if actor_ref.ability_deactivated_from_death(game) {return}
        if priority != Priority::Investigative {return;}

        let mut evil_count: u8 = 0;
        for player in PlayerReference::all_players(game)
            .filter(|player|player.alive(game))
            .filter(|player|VerdictsToday::player_guiltied_today(game, player))
        {
            if TallyClerk::player_is_suspicious(game, player){
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
        actor_ref.push_night_message(game, message);
    }
}

impl TallyClerk {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town)
        }
    }
}