use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::{chat::ChatMessageVariant, components::verdicts_today::VerdictsToday};
use crate::game::resolution_state::ResolutionState;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct FlowerGirl;

pub type ClientRoleState = FlowerGirl;

impl RoleStateImpl<ClientRoleState> for FlowerGirl {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_blocked(game) {return}
        if priority != Priority::Investigative {return;}

        let mut evil_count = 0;
        for player in PlayerReference::all_players(game).into_iter()
            .filter(|player|player.alive(game))
            .filter(|player|VerdictsToday::player_guiltied_today(game, player))
        {
            if FlowerGirl::player_is_suspicious(game, player){
                evil_count += 1;
            }
        }
        
        let message = ChatMessageVariant::FlowerGirlResult{ evil_count };
        actor_ref.push_night_message(game, message);
    }
}

impl FlowerGirl {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            !player_ref.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town)
        }
    }
}