use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{GraveKiller, GraveReference};
use crate::game::phase::PhaseType;
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
    pub retrains_remaining: u8
}
impl Default for Retrainer {
    fn default() -> Self {
        Self {
            backup: None,
            retrains_remaining: 2
        }
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Retrainer {
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
                    target_ref.try_night_kill(
                        backup, game, GraveKiller::Faction(Faction::Mafia), 1, false
                    );
                }
                backup.set_night_visits(game, visits);
            }
            
        } else if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
    
            target_ref.try_night_kill(
                actor_ref, game, GraveKiller::Faction(Faction::Mafia), 1, false
            );
        }        
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        game.day_number() > 1
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.backup {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
            } else {
                actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: Some(target_ref), ..self}));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: Some(target_ref), ..self}));
        }

        let RoleState::Retrainer(Retrainer { backup, .. }) = *actor_ref.role_state(game) else {
            unreachable!("Role was just set to Retrainer");
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
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
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
            backup.set_role(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
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
                    backup.set_role(game, role.default_state());
                    retrainer.retrains_remaining = retrainer.retrains_remaining.saturating_sub(1);
                }
            }
            
            actor_ref.set_role_state(game, RoleState::Retrainer(retrainer));
        }
    }
}