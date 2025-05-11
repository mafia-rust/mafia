use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::components::confused::Confused;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{chat::ChatMessageVariant, components::verdicts_today::VerdictsToday};
use crate::game::game_conclusion::GameConclusion;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::detective::Detective;
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

        let evil_count = Self::result(game, midnight_variables, actor_ref);
        
        let message = ChatMessageVariant::TallyClerkResult{ evil_count };
        actor_ref.push_night_message(midnight_variables, message);
    }
}

impl TallyClerk {
    fn result(game: &Game, midnight_variables: &MidnightVariables, actor_ref: PlayerReference)->u8{
        let confused = Confused::is_confused(game, actor_ref);
        #[expect(clippy::cast_possible_truncation, reason = "max 255 players")]
        let out = PlayerReference::all_players(game)
            .filter(|player|
                player.alive(game) &&
                VerdictsToday::player_guiltied_today(game, player) &&
                if confused {
                    Self::player_is_suspicious_confused(game, midnight_variables, *player, actor_ref)
                } else {
                    Self::player_is_suspicious(game, midnight_variables, *player)
                }
            ).count() as u8;
        out
    }
    fn player_is_suspicious(game: &Game, midnight_variables: &MidnightVariables, player_ref: PlayerReference) -> bool {
        if player_ref.has_suspicious_aura(game, midnight_variables){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town)
        }
    }
    pub fn player_is_suspicious_confused(game: &Game, midnight_variables: &MidnightVariables, player_ref: PlayerReference, actor_ref: PlayerReference) -> bool {
        Detective::player_is_suspicious_confused(game, midnight_variables, player_ref, actor_ref)
    }
}
