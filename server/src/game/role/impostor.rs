use serde::Serialize;

use crate::game::ability_input::AbilityInput;
use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveInformation;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::godfather::Godfather;
use super::{Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Impostor{
    pub backup: Option<PlayerReference>,
    pub fake_role: Role,
}

impl Default for Impostor {
    fn default() -> Self {
        Self {
            backup: None,
            fake_role: Role::Jester,
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Impostor {
    type ClientRoleState = Impostor;
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
                actor_ref.set_role_state(game, Impostor{backup: None, ..self});
            } else {
                actor_ref.set_role_state(game, Impostor{backup: Some(target_ref), ..self});
            }
        } else {
            actor_ref.set_role_state(game, Impostor{backup: Some(target_ref), ..self});
        }

        let RoleState::Impostor(Impostor { backup, .. }) = *actor_ref.role_state(game) else {
            unreachable!("Role was just set to Impostor");
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
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave: crate::game::grave::GraveReference) {
        if grave.deref(game).player == actor_ref {
            let grave = grave.deref_mut(game);
            grave.information = match grave.information.clone() {
                GraveInformation::Obscured => GraveInformation::Obscured,
                GraveInformation::Normal {will, death_cause, death_notes, .. } => {
                    GraveInformation::Normal { role: self.fake_role, will, death_cause, death_notes } 
                },
            };
            
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){

        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, RoleState::Impostor(Impostor{backup: None, ..self}));
            for player_ref in PlayerReference::all_players(game){
                if InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref){
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to godfather
            backup.set_role_and_win_condition_and_revealed_group(game, RoleState::Impostor(Impostor{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Impostor(Impostor{backup: None, ..self}));
        }
    }
    fn on_ability_input_received(mut self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: crate::game::ability_input::AbilityInput) {
        if actor_ref != input_player {return;}
        if !actor_ref.alive(game) {return};
        match ability_input {
            AbilityInput::Disguiser{selection } => {
                if let Some(target) = selection.0 {
                    self.fake_role = target;
                }
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}


impl Impostor {
    pub fn set_role(game: &mut Game, actor_ref: PlayerReference, role: Role){
        if !actor_ref.alive(game) {
            return;
        }


        if let RoleState::Impostor(mut impostor) = actor_ref.role_state(game).clone() {

            impostor.fake_role = role;
            actor_ref.set_role_state(game, RoleState::Impostor(impostor));
        }
    }
}