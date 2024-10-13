use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::role_list::{Faction, RoleSet};
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Retrainer{
    pub backup: Option<PlayerReference>,
    pub retrains_remaining: u8,
    attack_target: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: RetrainerAction
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RetrainerAction{
    SetBackup{backup: Option<PlayerReference>},
    SetAttack{target: Option<PlayerReference>},
    Retrain{role: Role}
}

impl Default for Retrainer {
    fn default() -> Self {
        Self {
            backup: None,
            retrains_remaining: 2,
            attack_target: None
        }
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Retrainer {
    type ClientRoleState = Retrainer;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if priority != Priority::Kill {return}
        if game.day_number() == 1 {return}
        
        if actor_ref.night_blocked(game) {
            if let Some(backup) = self.backup {

                let mut visits = backup.night_visits(game).clone();
                if let Some(visit) = visits.first_mut(){
                    visit.attack = true;
                    let target_ref = visit.target;
            
                    game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackupKilled { backup: backup.index() });
                    target_ref.try_night_kill_single_attacker(
                        backup, game, GraveKiller::Faction(Faction::Mafia), AttackPower::Basic, false
                    );
                }
                backup.set_night_visits(game, visits);
            }
            
        } else if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
    
            target_ref.try_night_kill_single_attacker(
                actor_ref, game, GraveKiller::Faction(Faction::Mafia), AttackPower::Basic, false
            );
        }        
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        match action_choice.action {
            RetrainerAction::SetBackup { backup } => {
                self.backup = match backup {
                    Some(backup) => {
                        if !(
                            actor_ref != backup &&
                            actor_ref.alive(game) &&
                            backup.alive(game) &&
                            backup.role(game).faction() == Faction::Mafia
                        ){
                            return;
                        }
                        Some(backup)
                    },
                    None => {
                        None
                    }
                };
                game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackup { backup: self.backup.clone().map(|p|p.index()) });
                actor_ref.set_role_state(game, self);

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
            },
            RetrainerAction::SetAttack { target } => {
                if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
                self.attack_target = match target {
                    Some(target) => {
                        if !(
                            crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, Some(target), false) &&
                            game.day_number() > 1
                        ){
                            return;
                        }
                        Some(target)
                    },
                    None => {
                        None
                    },
                };
                actor_ref.set_role_state(game, self);
            },
            RetrainerAction::Retrain { role } => {
                Retrainer::retrain(game, actor_ref, role);
            },
        }
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(self.attack_target, true)
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){

        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia{
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to godfather
            backup.set_role_and_wincon(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: crate::game::phase::PhaseType) {
        actor_ref.set_role_state(game, Retrainer{
            attack_target: None,
            ..self
        });
    }
}


impl Retrainer {
    pub fn retrain(game: &mut Game, actor_ref: PlayerReference, role: Role){
        if
            (!RoleSet::MafiaSupport.get_roles().contains(&role) && role != Role::MafiaSupportWildcard) || 
            !actor_ref.alive(game) ||
            game.current_phase().is_night()
        {
            return;
        }


        if let RoleState::Retrainer(mut retrainer) = actor_ref.role_state(game).clone() {

            if let Some(backup) = retrainer.backup {
                if retrainer.retrains_remaining > 0 && backup.role(game) != role{
                    backup.set_role_and_wincon(game, role.default_state());
                    retrainer.retrains_remaining = retrainer.retrains_remaining.saturating_sub(1);
                }
            }
            
            actor_ref.set_role_state(game, RoleState::Retrainer(retrainer));
        }
    }
}