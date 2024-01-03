use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleState, Role, RoleStateImpl};


#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Jailor { 
    jailed_target_ref: Option<PlayerReference>, 
    executions_remaining: u8
}

impl Default for Jailor {
    fn default() -> Self {
        Self { 
            jailed_target_ref: None, 
            executions_remaining: 3
        }
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownPower;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Jailor {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill { return; } 
    
        if let Some(visit) = actor_ref.night_visits(game).first() {
    
            let target_ref = visit.target;
            if target_ref.night_jailed(game){
                target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Jailor), 3, false);

                self.executions_remaining = if target_ref.role(game).faction_alignment().faction() == Faction::Town { 0 } else { self.executions_remaining - 1 };
                self.jailed_target_ref = None;
                actor_ref.set_role_state(game, RoleState::Jailor(self));
            }
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        target_ref.night_jailed(game) && 
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref != target_ref && 
        actor_ref.alive(game) && 
        target_ref.alive(game) && 
        game.phase_machine.day_number > 1 &&
        self.executions_remaining > 0
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.jailed_target_ref {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Jailor(Jailor { jailed_target_ref: None, executions_remaining: self.executions_remaining}));
            } else {
                actor_ref.set_role_state(game, RoleState::Jailor(Jailor { jailed_target_ref: Some(target_ref), executions_remaining: self.executions_remaining }))
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Jailor(Jailor { jailed_target_ref: Some(target_ref), executions_remaining: self.executions_remaining }))
        }
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {        
        game.current_phase().is_day() &&
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        self.executions_remaining > 0
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, 
            if PlayerReference::all_players(game).into_iter().any(|p|p.night_jailed(game)) {
                vec![ChatGroup::Jail]
            }else{
                vec![]
            }
        )
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);
        if 
            game.current_phase().is_night() &&
            actor_ref.alive(game) &&
            PlayerReference::all_players(game).into_iter().any(|p|p.night_jailed(game))
        {
            out.push(ChatGroup::Jail);
        }
        out
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    
        if phase != PhaseType::Night{return;}
        
        if let Some(jailed_ref) = self.jailed_target_ref {
            if jailed_ref.alive(game) && actor_ref.alive(game){
        
                jailed_ref.set_night_jailed(game, true);
                actor_ref.add_chat_message(game, 
                    ChatMessage::JailedTarget{ player_index: jailed_ref.index() }
                );
            }
        }
        self.jailed_target_ref = None;
        actor_ref.set_role_state(game, RoleState::Jailor(self));
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}