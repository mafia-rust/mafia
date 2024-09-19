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
#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Wizard{
    pub level: u8,
    pub action: WizardAction,
    pub last_used_action: Option<WizardAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "camelCase")]
pub enum WizardAction{
    #[default]
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
            
            if priority == Priority::FinalPriority {
                actor_ref.set_role_state(game, 
                    RoleState::Wizard(Wizard{
                        level: self.level + 1,                        
                        ..self.clone()
                    })
                );
            }

            match self.action {
                WizardAction::Meditate => Wizard::spell_meditate(self, game, priority, actor_ref),
                WizardAction::Poison => Wizard::spell_poison(self, game, priority, actor_ref, target),
                WizardAction::Shield => Wizard::spell_shield(self, game, priority, actor_ref),
                WizardAction::Illusion => Wizard::spell_illusion(self, game, priority, actor_ref),
                WizardAction::Illuminate => Wizard::spell_illuminate(self, game, priority, actor_ref),
                WizardAction::Absorb => Wizard::spell_absorb(self, game, priority, actor_ref),
                WizardAction::Reflect => Wizard::spell_reflect(self, game, priority, actor_ref),
                WizardAction::Pyrolyze => Wizard::spell_pyrolyze(self, game, priority, actor_ref),
                WizardAction::Polymorph => Wizard::spell_polymorph(self, game, priority, actor_ref),
                WizardAction::Smite => Wizard::spell_smite(self, game, priority, actor_ref),
                WizardAction::Ascend => Wizard::spell_ascend(self, game, priority, actor_ref),
                _ => {}
            }
        }

    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        match self.action {
            WizardAction::Meditate => false,
            WizardAction::Poison => true,
            WizardAction::Shield => false,
            WizardAction::Illusion => true,
            WizardAction::Illuminate => true,
            WizardAction::Absorb => false,
            WizardAction::Reflect => false,
            WizardAction::Pyrolyze => true,
            WizardAction::Polymorph => true,
            WizardAction::Smite => true,
            WizardAction::Ascend => false,
        }

    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {

        let attack = match self.action {
            WizardAction::Smite => true,
            _ => false
        };
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, attack)
    }
}


impl Wizard {
    fn spell_meditate(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        if priority == Priority::FinalPriority {
            actor_ref.set_role_state(game, RoleState::Wizard(Wizard{
                level: self.level + 1,
                last_used_action: Some(WizardAction::Meditate),
                ..self
            }));

            
        }
    }

    fn spell_poison(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference, target_ref: PlayerReference) {
        println!("Poisoning");
    }

    fn spell_shield(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Shielding");
    }

    fn spell_illusion(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Illusioning");
    }

    fn spell_illuminate(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Illuminating");
    }

    fn spell_absorb(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Absorbing");
    }

    fn spell_reflect(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Reflecting");
    }

    fn spell_pyrolyze(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Pyrolyzing");
    }

    fn spell_polymorph(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Polymorphing");
    }

    fn spell_smite(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Smiting");
    }

    fn spell_ascend(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) {
        println!("Ascending");
    }

    
}
