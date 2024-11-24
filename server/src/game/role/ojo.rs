use serde::Serialize;

use crate::game::ability_input::common_selection::two_role_outline_option_selection::TwoRoleOutlineOptionSelection;
use crate::game::ability_input::AbilityInput;
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
    pub chosen_outline: TwoRoleOutlineOptionSelection,
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


                if let Some(chosen_outline) = self.chosen_outline.0{
                    let result = Self::get_result(game, chosen_outline);
                    actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                        role_outline: chosen_outline.deref(&game).clone(),
                        result: result.clone()
                    });
                    self.previously_given_results.push((chosen_outline, result));
                }
        
                if let Some(chosen_outline) = self.chosen_outline.1{
                    let result = Self::get_result(game, chosen_outline);
                    actor_ref.push_night_message(game, ChatMessageVariant::AuditorResult {
                        role_outline: chosen_outline.deref(&game).clone(),
                        result: result.clone()
                    });
                    self.previously_given_results.push((chosen_outline, result));
                }
        
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
    fn on_ability_input_received(mut self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: crate::game::ability_input::AbilityInput) {
        if actor_ref != input_player {return;}
        if !actor_ref.alive(game) {return};
        match ability_input {
            AbilityInput::OjoInvestigate { selection } => {                   
                if let Some(outline) = selection.0{
                    if !self.previously_given_results.iter().any(|(i, _)| *i == outline) {
                        self.chosen_outline.0 = Some(outline);
                    }
                }else{
                    self.chosen_outline.0 = None;
                }
                if let Some(outline) = selection.1{
                    if !self.previously_given_results.iter().any(|(i, _)| *i == outline) {
                        self.chosen_outline.1 = Some(outline);
                    }
                }else{
                    self.chosen_outline.1 = None;
                }
                
                if self.chosen_outline.0.is_some() && self.chosen_outline.1 == self.chosen_outline.0{
                    self.chosen_outline.1 = None;
                }

                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
    fn convert_selection_to_visits(self, game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        let mut out = vec![];
        if let Some(chosen_outline) = self.chosen_outline.0{
            let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
            out.push(Visit{ target: player, attack: false });
        }
        if let Some(chosen_outline) = self.chosen_outline.1{
            let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
            out.push(Visit{ target: player, attack: false });
        }

        if game.day_number() > 1 {
            if let Some(role) = self.role_chosen {
                for player in PlayerReference::all_players(game){
                    if player.alive(game) && player.role(game) == role {
                        out.push(Visit{ target: player, attack: true });
                    }
                }
            }
        }

        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Obituary => {
                self.chosen_outline = TwoRoleOutlineOptionSelection(None, None);
                self.role_chosen = None;
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}

impl Ojo{
    //panics if chosen_outline is not found
    pub fn get_result(game: &Game, chosen_outline: RoleOutlineReference) -> AuditorResult {
        let (role, _) = chosen_outline.deref_as_role_and_player_originally_generated(game);
        AuditorResult::One{role}
    }
}