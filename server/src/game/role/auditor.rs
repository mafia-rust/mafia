use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use rand::prelude::SliceRandom;
use super::{Priority, Role, RoleStateImpl};


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


pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Auditor {
    type ClientRoleState = Auditor;
    type RoleActionChoice = super::common_role::CommonRoleActionChoice;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if priority != Priority::Investigative {return;}
        if actor_ref.night_blocked(game) {return;}

        let Some(chosen_outline) = self.chosen_outline else {return;};

        let (role, _) = match game.roles_originally_generated.get(chosen_outline as usize) {
            Some(map) => *map,
            None => unreachable!("Auditor role outline not found")
        };
        
        let outline = match game.settings.role_list.0.get(chosen_outline as usize){
            Some(outline) => outline,
            None => unreachable!("Auditor role outline not found")
        };

        let result = if outline.get_roles().len() == 1 || outline.get_roles().len() == 2 {
            AuditorResult::One{role}
        }else{
            let fake_role = outline
                .get_roles()
                .into_iter()
                .filter(|x|game.settings.enabled_roles.contains(x))
                .filter(|x|*x != role)
                .collect::<Vec<Role>>()
                .choose(&mut rand::thread_rng())
                .cloned();

            if let Some(fake_role) = fake_role{
                let mut two = [role, fake_role];
                two.shuffle(&mut rand::thread_rng());
                AuditorResult::Two{roles: [two[0], two[1]]}
            } else {
                AuditorResult::One{role}
            }
        };

        let message = ChatMessageVariant::AuditorResult {
            role_outline: outline.clone(),
            result: result.clone()
        };
        self.previously_given_results.push((chosen_outline, result));
        
        actor_ref.push_night_message(game, message);
        actor_ref.set_role_state(game, super::RoleState::Auditor(self));
            
    }
    fn convert_selection_to_visits(self, game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        let Some(chosen_outline) = self.chosen_outline else {return vec![]};

        let (_, player) = match game.roles_originally_generated.get(chosen_outline as usize) {
            Some(map) => *map,
            None => unreachable!("Auditor role outline not found")
        };

        vec![Visit{ target: player, attack: false }]
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
}