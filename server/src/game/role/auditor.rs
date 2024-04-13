use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use rand::prelude::SliceRandom;
use super::{Priority, Role, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Auditor{
    pub chosen_outline: Option<u8>,
    pub previously_given_results: Vec<(u8, AuditorResult)>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AuditorResult{
    Two{roles: [Role; 2]},
    One{role: Role}
}

impl RoleStateImpl for Auditor {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    


    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if priority != Priority::Investigative {return;}
        if actor_ref.night_roleblocked(game) {return;}

        let Some(chosen_outline) = self.chosen_outline else {return;};

        let (role, _) = match game.roles_to_players.get(chosen_outline as usize) {
            Some(map) => map.clone(),
            None => unreachable!("Auditor role outline not found")
        };
        
        let outline = match game.settings.role_list.0.get(chosen_outline as usize){
            Some(outline) => outline,
            None => unreachable!("Auditor role outline not found")
        };
        let fake_role = outline
            .get_roles()
            .into_iter()
            .filter(|x|!game.settings.excluded_roles.contains(x))
            .collect::<Vec<Role>>()
            .choose(&mut rand::thread_rng())
            .cloned();

        let result = if let Some(fake_role) = fake_role{
            if fake_role != role{
                let mut two = [role, fake_role];
                two.shuffle(&mut rand::thread_rng());
                AuditorResult::Two{roles: [two[0], two[1]]}
            }else{
                AuditorResult::One{role}
            }
        } else {
            AuditorResult::One{role}
        };

        let message = ChatMessageVariant::AuditorResult {
            role_outline: outline.clone(),
            result: result.clone()
        };
        self.previously_given_results.push((chosen_outline, result));
        
        actor_ref.push_night_message(game, message);
        actor_ref.set_role_state(game, super::RoleState::Auditor(self));
            
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        let Some(chosen_outline) = self.chosen_outline else {return vec![]};

        let (_, player) = match game.roles_to_players.get(chosen_outline as usize) {
            Some(map) => map.clone(),
            None => unreachable!("Auditor role outline not found")
        };

        vec![Visit{ target: player, attack: false }]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Obituary => {
                self.chosen_outline = None;
                actor_ref.set_role_state(game, super::RoleState::Auditor(self));
            },
            _ => {}
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}