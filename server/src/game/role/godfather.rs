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
use super::{Priority, RoleStateImpl, RoleState};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Godfather{
    backup: Option<PlayerReference>,
    attack_target: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: GodfatherAction
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum GodfatherAction{
    SetBackup{backup: Option<PlayerReference>},
    SetAttack{target: Option<PlayerReference>}
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Godfather {
    type ClientRoleState = Godfather;
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
            GodfatherAction::SetBackup { backup } => {
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
                
                game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackup{
                    backup: self.backup.clone().map(|p|p.index())
                });
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
            GodfatherAction::SetAttack { target } => {
                if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
                if !(
                    crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, target, false) &&
                    game.day_number() > 1
                ) {
                    return;
                }

                self.attack_target = target;
                actor_ref.set_role_state(game, self);
                return;
            },
        }
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(self.attack_target, true)
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){

        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, Godfather{backup: None, ..self});
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia{
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to godfather

            backup.set_role_and_wincon(game, RoleState::Godfather(Godfather{backup: None, attack_target: None}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Godfather(Godfather{backup: None, ..self}));
        }
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        match phase {
            PhaseType::Obituary => {
                self.attack_target = None;
                actor_ref.set_role_state(game, self);
            }
            _ => {}
        }
    }
}