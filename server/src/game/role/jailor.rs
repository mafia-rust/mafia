use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, Role, RoleStateImpl};


#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Jailor { 
    jailed_target_ref: Option<PlayerReference>, 
    should_attack: bool,
    executions_remaining: u8
}

impl Default for Jailor {
    fn default() -> Self {
        Self { 
            jailed_target_ref: None, 
            should_attack: false,
            executions_remaining: 3
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: JailorAction
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JailorAction {
    #[serde(rename_all = "camelCase")]
    Attack{should_attack: bool},
    Jail{player: PlayerReference}
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Jailor {
    type ClientRoleState = Jailor;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {


        match priority {
            Priority::Ward => {
                for player in PlayerReference::all_players(game){
                    if player.night_jailed(game){
                        player.ward(game);
                    }
                }
            }
            Priority::Roleblock => {
                for player in PlayerReference::all_players(game){
                    if player.night_jailed(game){
                        player.roleblock(game, false);
                    }
                }
            },
            Priority::Kill => {
                if let Some(visit) = actor_ref.night_visits(game).first() {
    
                    let target_ref = visit.target;
                    if target_ref.night_jailed(game){
                        target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Jailor), AttackPower::ProtectionPiercing, false);
        
                        self.executions_remaining = 
                            if target_ref.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town) {0} else {self.executions_remaining - 1};
                        self.jailed_target_ref = None;
                        actor_ref.set_role_state(game, RoleState::Jailor(self));
                    }
                }
            },
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        match action_choice.action {
            JailorAction::Attack { should_attack } => {
                self.should_attack = if !(
                    actor_ref.alive(game) &&
                    self.jailed_target_ref.is_some() &&
                    game.day_number() > 1 &&
                    self.executions_remaining > 0 &&
                    game.current_phase().phase() == crate::game::phase::PhaseType::Night
                ){
                    false
                }else{
                    should_attack
                };

                actor_ref.set_role_state(game, self);
            },
            JailorAction::Jail { player } => {
                self.jailed_target_ref = if !(
                    game.current_phase().is_day() &&
                    actor_ref != player &&
                    actor_ref.alive(game) &&
                    player.alive(game)
                ) {
                    None
                }else{
                    Some(player)
                };
                actor_ref.set_role_state(game, self);
            },
        }
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(self.jailed_target_ref, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, 
            if PlayerReference::all_players(game).any(|p|p.night_jailed(game)) {
                vec![ChatGroup::Jail].into_iter().collect()
            }else{
                vec![]
            }
        )
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);
        if 
            game.current_phase().is_night() &&
            actor_ref.alive(game) &&
            PlayerReference::all_players(game).any(|p|p.night_jailed(game))
        {
            out.insert(ChatGroup::Jail);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
    
        if phase != PhaseType::Night{return;}
        
        if let Some(jailed_ref) = self.jailed_target_ref {
            if jailed_ref.alive(game) && actor_ref.alive(game){
        
                jailed_ref.set_night_jailed(game, true);
                actor_ref.add_private_chat_message(game, 
                    ChatMessageVariant::JailedTarget{ player_index: jailed_ref.index() }
                );
            }
        }
        self.jailed_target_ref = None;
        self.should_attack = false;
        actor_ref.set_role_state(game, RoleState::Jailor(self));
    }
}