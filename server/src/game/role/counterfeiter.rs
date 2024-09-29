use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterfeiter{
    backup: Option<PlayerReference>,
    
    pub fake_role: Role,
    pub fake_will: String,
    pub forges_remaining: u8,
    pub forged_ref: Option<PlayerReference>,

    pub action: CounterfeiterAction
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState{
    backup: Option<PlayerReference>,
    
    pub fake_role: Role,
    pub fake_will: String,
    pub forges_remaining: u8,

    pub action: CounterfeiterAction
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CounterfeiterAction{
    Forge,
    NoForge
}
impl Default for Counterfeiter {
    fn default() -> Self {
        Counterfeiter {
            backup: None,

            forges_remaining: 3,
            forged_ref: None,
            fake_role: Role::Jester,
            fake_will: "".to_owned(),

            action: CounterfeiterAction::NoForge,
        }
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Counterfeiter {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        
        if game.day_number() == 1 {return}

        if actor_ref.night_blocked(game) {
            if let Some(backup) = self.backup {
                match priority {
                    Priority::Deception => {
                        let mut visits = backup.night_visits(game).clone();
                        if let Some(visit) = visits.first_mut() {
                            visit.attack = true;
                            game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackupKilled { backup: backup.index() });
                        }
                        backup.set_night_visits(game, visits);
                    },
                    Priority::Kill => {
                        if let Some(visit) = backup.night_visits(game).first(){
                            let target_ref = visit.target;
                            target_ref.try_night_kill_single_attacker(
                                backup, game, GraveKiller::Faction(Faction::Mafia), AttackPower::Basic, false
                            );
                        }
                    },
                    _ => {}
                }
            }
        }else{
            match priority {
                Priority::Deception => {
                    if self.forges_remaining == 0 || self.action == CounterfeiterAction::NoForge {return}

                    let Some(visit) = actor_ref.night_visits(game).first() else{return};
                    let target_ref = visit.target;
    
                    target_ref.set_night_grave_role(game, Some(self.fake_role));
                    target_ref.set_night_grave_will(game, self.fake_will.clone());
                    actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter { 
                        forges_remaining: self.forges_remaining - 1, 
                        forged_ref: Some(target_ref), 
                        ..self
                    }));
                },
                Priority::Kill => {
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;
                
                        target_ref.try_night_kill_single_attacker(
                            actor_ref, game, GraveKiller::Faction(Faction::Mafia), AttackPower::Basic, false
                        );
                    }
                },
                Priority::Investigative => {
                    if let Some(forged_ref) = self.forged_ref {
                        if forged_ref.night_died(game) {
                            actor_ref.push_night_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                                player: forged_ref,
                                role: forged_ref.role(game),
                                will: forged_ref.will(game).to_string(),
                            });
                        }
                    }
                },
                _ => {}
            }
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        game.day_number() > 1
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.backup {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{backup: None, ..self}));
            } else {
                actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{backup: Some(target_ref), ..self}));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{backup: Some(target_ref), ..self}));
        }

        let RoleState::Counterfeiter(Counterfeiter { backup, .. }) = *actor_ref.role_state(game) else {
            unreachable!("Role was just set to Counterfeiter");
        };

        game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackup { backup: backup.map(|p|p.index()) });

        for player_ref in PlayerReference::all_players(game){
            if player_ref.role(game).faction() != Faction::Mafia{
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        if let Some(backup) = backup {
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia {
                    continue;
                }
                player_ref.push_player_tag(game, backup, Tag::GodfatherBackup);
            }
        }
        
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        target_ref.role(game).faction() == Faction::Mafia
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{
            forged_ref: None,
            ..self
        }));
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{backup: None, ..self.clone()}));
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia{
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to Counterfeiter
            backup.set_role(game, RoleState::Counterfeiter(Counterfeiter{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Counterfeiter(Counterfeiter{backup: None, ..self}));
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Counterfeiter {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            backup: self.backup,
            fake_role: self.fake_role,
            fake_will: self.fake_will,
            forges_remaining: self.forges_remaining,
            action: self.action,
        }
    }
}