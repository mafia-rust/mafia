use serde::{Deserialize, Serialize};


use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wizard{
    pub level: u8,
    pub action: WizardAction,
}


impl Wizard {
    fn spell_meditate(self, game: &mut Game, actor_ref: PlayerReference) {
        actor_ref.set_role_state(game, RoleState::Wizard(Wizard{
            level: self.level + 1,
            action: self.action
        }))
    }

    fn spell_poison(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_shield(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_illusion(self, game: &mut Game, actor_ref: PlayerReference,    target_ref: PlayerReference) {
        
    }

    fn spell_illuminate(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_absorb(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
        
    }

    fn spell_reflect(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_pyrolyze(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_polymorph(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_smite(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }

    fn spell_ascend(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        
    }
    
}

impl Default for Wizard{
    fn default() -> Self {
        Self {
            level: 0,
    action: WizardAction::Meditate
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum WizardAction{
    Meditate,
    Poison,
    Shield,
    Illusion,
    Illuminate,
    Absorb,
    Reflect,
    Pyrolyze,
    Polymorph,
    Smite,
    Ascend,
}


impl RoleStateImpl for Wizard {
    
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target = visit.target;
            
            match self.action {
                WizardAction::Meditate => {
                    Wizard::spell_meditate(self, game, actor_ref);
                    return
                }
                WizardAction::Poison => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Shield => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Illusion => {
                    if priority != Priority::Kill {return;}
                    
                },
                WizardAction::Illuminate => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Absorb => {
                    if priority != Priority::Kill {return;}
                    
                },
                WizardAction::Reflect => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Pyrolyze => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Polymorph => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Smite => {
                    if priority != Priority::Kill {return;}
                },
                WizardAction::Ascend => {
                    if priority != Priority::Kill {return;}
                },

            }
        }
        self.level += 1;
    }
    
    





    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Night {
            self.action = WizardAction::Poison;
            actor_ref.set_role_state(game, RoleState::Wizard(self))
        }
    }
    
}

