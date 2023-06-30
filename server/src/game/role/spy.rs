use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Spy;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SpyBug{
    Silenced, Roleblocked, Protected, Transported
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownInvestigative;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Spy {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}

        match priority {
            Priority::Investigative => {
                if actor_ref.night_roleblocked(game) {return;}

                let mut mafia_visits = vec![];
                let mut coven_visits = vec![];
                for other_player in PlayerReference::all_players(game){
                    if other_player.role(game).faction_alignment().faction() == Faction::Mafia{
                        mafia_visits.append(&mut other_player.night_visits(game).iter().filter(|v|!v.astral).map(|v|v.target.index()).collect());
                    }else if other_player.role(game).faction_alignment().faction() == Faction::Coven{
                        coven_visits.append(&mut other_player.night_visits(game).iter().filter(|v|!v.astral).map(|v|v.target.index()).collect());
                    }
                }
                mafia_visits.shuffle(&mut rand::thread_rng());
                coven_visits.shuffle(&mut rand::thread_rng());
                
                actor_ref.push_night_message(game, ChatMessage::SpyMafiaVisit { players: mafia_visits });
                actor_ref.push_night_message(game, ChatMessage::SpyCovenVisit { players: coven_visits });
            },
            Priority::SpyBug => {
                let Some(visit) = actor_ref.night_visits(game).first()else{return};
            
                if visit.target.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed );
                    return
                }

                for message in visit.target.night_messages(game).clone(){
                    match message{
                        ChatMessage::Silenced => {
                            actor_ref.push_night_message(game, ChatMessage::SpyBug { bug: SpyBug::Silenced });
                        },
                        ChatMessage::RoleBlocked { immune: _ } =>{
                            actor_ref.push_night_message(game, ChatMessage::SpyBug { bug: SpyBug::Roleblocked });
                        }
                        ChatMessage::ProtectedYou => {
                            actor_ref.push_night_message(game, ChatMessage::SpyBug { bug: SpyBug::Protected });
                        }
                        ChatMessage::Transported => {
                            actor_ref.push_night_message(game, ChatMessage::SpyBug { bug: SpyBug::Transported });
                        }
                        _=>{}
                    }
                };
            },
            _=>{}
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
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}