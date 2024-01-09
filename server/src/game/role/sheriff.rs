use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl};

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownInvestigative;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Sheriff;

impl RoleStateImpl for Sheriff {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}

        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
            if visit.target.night_jailed(game){
                actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                return
            }
            let message = ChatMessage::SheriffResult {
                suspicious: Sheriff::player_is_suspicious(game, visit.target)
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
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
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}

impl Sheriff {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.doused(game) || player_ref.night_framed(game) {return true;}

        match player_ref.role(game).faction_alignment().faction() {
            Faction::Town => false,
            _ => match player_ref.role(game) {
                //exceptions
                super::Role::Godfather => false,
                super::Role::Jester => false,
                super::Role::Executioner => false,
                super::Role::Werewolf => if game.day_number() == 1 || game.day_number() == 3 {false} else {true},
                _ => true
            }
        }
    }
}