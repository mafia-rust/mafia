use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleState, Role, RoleStateImpl};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Veteran { 
    alerts_remaining: u8, 
    alerting_tonight: bool 
}

impl Default for Veteran {
    fn default() -> Self {
        Veteran {
            alerts_remaining: 3,
            alerting_tonight: false
        }
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Veteran {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
        
        match priority {
            Priority::TopPriority => {
                if self.alerts_remaining > 0 {
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        if visit.target == actor_ref{
                            actor_ref.set_role_state(game, RoleState::Veteran(Veteran { 
                                alerts_remaining: self.alerts_remaining - 1, 
                                alerting_tonight: true 
                            }));
                        }
                    }
                }
            }
            Priority::Heal=>{
                if !self.alerting_tonight {return}
                actor_ref.increase_defense_to(game, 1);
            }
            Priority::Kill => {
                if !self.alerting_tonight {return}

                for other_player_ref in PlayerReference::all_players(game)
                    .into_iter()
                    .filter(|other_player_ref|
                        *other_player_ref != actor_ref &&
                        other_player_ref.night_visits(game)
                            .iter()
                            .any(|v|!v.astral&&v.target==actor_ref)
                    ).collect::<Vec<PlayerReference>>()
                {
                    other_player_ref.push_night_message(game,
                        ChatMessage::VeteranAttackedYou 
                    );

                    other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Veteran), 2, false);
                    
                    actor_ref.push_night_message(game, 
                        ChatMessage::VeteranAttackedVisitor 
                    );
                }
            }
            _=>{}
        }
    }
    
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        self.alerts_remaining > 0 &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {

    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, true, false)
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
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Veteran(Veteran { alerts_remaining: self.alerts_remaining, alerting_tonight: false }));   
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}