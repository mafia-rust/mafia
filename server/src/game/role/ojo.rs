use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::phase::PhaseType;
use crate::game::role_outline_reference::RoleOutlineReference;
use crate::game::visit::Visit;
use crate::game::{attack_power::AttackPower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::Game;
use super::auditor::AuditorResult;
use super::{Priority, RoleStateImpl, Role};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ojo{
    pub role_chosen: Option<Role>,
    pub chosen_outline: Option<RoleOutlineReference>,
    pub previously_given_results: Vec<(RoleOutlineReference, AuditorResult)>,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Ojo {
    type ClientRoleState = Ojo;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if actor_ref.night_blocked(game) {return;}

        match priority {
            Priority::Kill => {
                if game.day_number() == 1 {return;}
                for visit in actor_ref.night_visits(game).clone() {
                    if visit.attack {
                        visit.target.try_night_kill_single_attacker(
                            actor_ref, game, 
                            GraveKiller::Role(Role::Ojo), 
                            AttackPower::ArmorPiercing, 
                            true
                        );
                    }
                }
            },
            Priority::Investigative => {
                let visited_me = actor_ref.all_visitors(game);

                for player in PlayerReference::all_players(game) {
                    if visited_me.contains(&player) {
                        actor_ref.insert_role_label(game, player);
                    }
                }

                let Some(chosen_outline) = self.chosen_outline else {return;};

                let (role, _) = chosen_outline.deref_as_role_and_player_originally_generated(game);
                
                let outline = chosen_outline.deref(&game).clone();

                let result =  AuditorResult::One{role};
                
                actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                    role_outline: outline,
                    result: result.clone()
                });
                
                self.previously_given_results.push((chosen_outline, result));
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
    fn convert_selection_to_visits(self, game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        let mut all_visits = Vec::new();

        if let Some(chosen_outline) = self.chosen_outline {
            let (_, audited_player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
            all_visits.push(Visit{ target: audited_player, attack: false });
        }


        if game.day_number() > 1 {
            if let Some(role) = self.role_chosen {
                for player in PlayerReference::all_players(game){
                    if player.alive(game) && player.role(game) == role {
                        all_visits.push(Visit{ target: player, attack: true });
                    }
                }
            }
        }
        
        all_visits
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Obituary => {
                self.chosen_outline = None;
                self.role_chosen = None;
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}