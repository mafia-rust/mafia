use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::player::PlayerReference;
use crate::game::role_list::RoleSet;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::godfather::Godfather;
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


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Retrainer {
    type ClientRoleState = Retrainer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        Godfather::night_ability(self.backup, game, actor_ref, priority);    
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        game.day_number() > 1
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.backup {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, Retrainer{backup: None, ..self});
            } else {
                actor_ref.set_role_state(game, Retrainer{backup: Some(target_ref), ..self});
            }
        } else {
            actor_ref.set_role_state(game, Retrainer{backup: Some(target_ref), ..self});
        }

        let RoleState::Retrainer(Retrainer { backup, .. }) = *actor_ref.role_state(game) else {
            unreachable!("Role was just set to Retrainer");
        };

        game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackup { backup: backup.map(|p|p.index()) });

        for player_ref in PlayerReference::all_players(game){
            if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref){
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        if let Some(backup) = backup {
            for player_ref in PlayerReference::all_players(game){
                if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref) {
                    continue;
                }
                player_ref.push_player_tag(game, backup, Tag::GodfatherBackup);
            }
        }
        
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        RoleSet::Mafia.get_roles().contains(&target_ref.role(game)) &&
        InsiderGroupID::Mafia.is_player_in_revealed_group(game, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){

        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
            for player_ref in PlayerReference::all_players(game){
                if InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref){
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to godfather
            backup.set_role_and_win_condition_and_revealed_group(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Retrainer(Retrainer{backup: None, ..self}));
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
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
                    backup.set_role_and_win_condition_and_revealed_group(game, role.default_state());
                    retrainer.retrains_remaining = retrainer.retrains_remaining.saturating_sub(1);
                }
            }
            
            actor_ref.set_role_state(game, RoleState::Retrainer(retrainer));
        }
    }
}