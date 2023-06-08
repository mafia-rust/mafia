use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::chat::night_message::NightInformation;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleState, Role, RoleStateImpl};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownPower;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

#[derive(Serialize, Clone, Debug)]
pub struct Jailor { 
    jailed_target_ref: Option<PlayerReference>, 
    executions_remaining: u8
}

impl Default for Jailor {
    fn default() -> Self {
        Self { 
            jailed_target_ref: Some(PlayerReference::new_unchecked(0)), 
            executions_remaining: Default::default()
        }
    }
}

impl RoleStateImpl for Jailor {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill { return; } 
    
        if let Some(visit) = actor_ref.night_visits(game).first() {
    
            let target_ref = visit.target;
            if target_ref.night_jailed(game){
                let killed = target_ref.try_night_kill(game, GraveKiller::Role(Role::Jailor), 3);
    
                if !killed {
                    actor_ref.push_night_messages(game, NightInformation::TargetSurvivedAttack);
                }
    
                let executions_remaining = if target_ref.role(game).faction_alignment().faction() == Faction::Town { 0 } else { self.executions_remaining - 1 };
                actor_ref.set_role_data(game, Jailor{ jailed_target_ref: None, executions_remaining });
            }
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let RoleState::Jailor{ executions_remaining, .. } = actor_ref.role_data(game) else {unreachable!()};
    
        target_ref.night_jailed(game) && 
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref != target_ref && 
        actor_ref.alive(game) && 
        target_ref.alive(game) && 
        game.phase_machine.day_number > 1 &&
        *executions_remaining > 0
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.jailed_target_ref {
            if old_target_ref == target_ref {
                actor_ref.set_role_data(game, RoleState::Jailor(Jailor { jailed_target_ref: None, executions_remaining: self.executions_remaining}));
            } else {
                actor_ref.set_role_data(game, RoleState::Jailor(Jailor { jailed_target_ref: Some(target_ref), executions_remaining: self.executions_remaining }))
            }
        } else {
            actor_ref.set_role_data(game, RoleState::Jailor(Jailor { jailed_target_ref: Some(target_ref), executions_remaining: self.executions_remaining }))
        }
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let RoleState::Jailor{ executions_remaining, .. } = actor_ref.role_data(game) else {unreachable!()};
        
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        *executions_remaining > 0
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
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    
        if phase != PhaseType::Night { return; }
    
        let RoleState::Jailor{ jailed_target_ref, executions_remaining } = actor_ref.role_data(game) else {unreachable!()};
        let executions_remaining = *executions_remaining;
        
        if let Some(jailed_ref) = jailed_target_ref.to_owned() {
            if jailed_ref.alive(game){
        
                jailed_ref.set_night_jailed(game, true);
                actor_ref.add_chat_message(game, 
                    ChatMessage::JailedTarget{ player_index: jailed_ref.index() }
                );
            }
        }
        actor_ref.set_role_data(game, RoleState::Jailor{ jailed_target_ref: None, executions_remaining });
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}