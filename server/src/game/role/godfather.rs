use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, RoleState};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Godfather{
    backup: Option<PlayerReference>
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::MafiaKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Godfather {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Mafia}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {Some(Team::Mafia)}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if priority != Priority::Kill {return}
        
        if actor_ref.night_jailed(game) || actor_ref.night_roleblocked(game) {
            if let Some(backup) = self.backup {
                if let Some(visit) = backup.night_visits(game).first(){
                    let target_ref = visit.target;
                    if target_ref.night_jailed(game){
                        backup.push_night_message(game, ChatMessage::TargetJailed);
                        return
                    }
            
                    target_ref.try_night_kill(backup, game, GraveKiller::Faction(Faction::Mafia), 1, true);
                }
            }
            
        } else {
            if let Some(visit) = actor_ref.night_visits(game).first(){
                let target_ref = visit.target;
                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }
        
                target_ref.try_night_kill(actor_ref, game, GraveKiller::Faction(Faction::Mafia), 1, true);
            }
        }        
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.backup {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Godfather(Godfather{backup: None}));
            } else {
                actor_ref.set_role_state(game, RoleState::Godfather(Godfather{backup: Some(target_ref)}));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Godfather(Godfather{backup: Some(target_ref)}));
        }

        let RoleState::Godfather(Godfather { backup }) = actor_ref.role_state(game) else {
            unreachable!("Role was just set to Godfather");
        };
        let backup = backup.clone();

        game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessage::GodfatherBackup { backup: backup.map(|p|p.index()) });

        for player_ref in PlayerReference::all_players(game){
            if player_ref.role(game).faction_alignment().faction() != Faction::Mafia{
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        if let Some(backup) = backup.clone() {
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction_alignment().faction() != Faction::Mafia {
                    continue;
                }
                player_ref.push_player_tag(game, backup, Tag::GodfatherBackup);
            }
        }
        
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        target_ref.role(game).faction_alignment().faction() == Faction::Mafia
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
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
        if actor_ref != dead_player_ref {return;}

        let Some(backup) = self.backup else {return};
        
        actor_ref.set_role_state(game, RoleState::Godfather(Godfather{backup: None}));

        for player_ref in PlayerReference::all_players(game){
            if player_ref.role(game).faction_alignment().faction() != Faction::Mafia{
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        //convert backup to godfather
        backup.set_role(game, RoleState::Godfather(Godfather{backup: None}));
        
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}