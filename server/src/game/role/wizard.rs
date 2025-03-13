use serde::{Deserialize, Serialize};

use crate::game::ability_input::ControllerParametersMap;
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::{GraveInformation, GraveReference};
use crate::game::{ability_input::ControllerID, attack_power::DefensePower};
use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::VecSet;
use super::{common_role, Priority, Role, RoleState, RoleStateImpl};

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;
#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Wizard{
    pub level: u8,
    pub action: WizardAction,
    pub last_used_action: Option<WizardAction>,
    pub tagged_for_obscure: VecSet<PlayerReference>,
    pub shielded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "camelCase")]
pub enum WizardAction{
    #[default]
    Meditate,
    Shield,
    Illusion,
    Reveal,
    Shock,
    Pyrolyze
}


impl RoleStateImpl for Wizard {
    type ClientRoleState = Self;
    
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
            
            if priority == Priority::StealMessages {
                self.level = self.level.saturating_add(1);
            }

            let wizard_state = match self.action {
                WizardAction::Meditate => Wizard::spell_meditate(self, game, priority, actor_ref),
                WizardAction::Shield => Wizard::spell_shield(self, game, priority, actor_ref),
                WizardAction::Illusion => Wizard::spell_illusion(self, game, priority, actor_ref),
                WizardAction::Reveal => Wizard::spell_reveal(self, game, priority, actor_ref),
                WizardAction::Pyrolyze => Wizard::spell_pyrolyze(self, game, priority, actor_ref),
                WizardAction::Shock => Wizard::spell_shock(self, game, priority, actor_ref),

            };

            actor_ref.set_role_state(game, RoleState::Wizard(wizard_state));


    }
    
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){
        if !actor_ref.alive(game) && grave_ref.deref(game).player != actor_ref {return}
        if !self.tagged_for_obscure.contains(&grave_ref.deref(game).player) && grave_ref.deref(game).player != actor_ref {return}
         
        actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
            player: grave_ref.deref(game).player,
            role: grave_ref.deref(game).player.role(game),
            will: grave_ref.deref(game).player.will(game).to_string(),
        });

        grave_ref.deref_mut(game).information = GraveInformation::Obscured;
    }

    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            false,
            ControllerID::role(actor_ref, Role::Wizard, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Wizard, 0),
            false
        )
    }
}


impl Wizard {
    fn spell_meditate(self, _game: &mut Game, priority: Priority, _actor_ref: PlayerReference) -> Wizard{
        if priority == Priority::StealMessages {
            return Wizard{
                level: self.level.saturating_add(1),
                last_used_action: Some(WizardAction::Meditate),
                ..self
            };
        }
        self
    }


    fn spell_shield(self, _game: &mut Game, priority: Priority, _actor_ref: PlayerReference) -> Wizard{
        if priority == Priority::Heal {
            return Wizard{
                shielded: true,
                last_used_action: Some(WizardAction::Shield),
                ..self
            };
        }
        self
    }

    fn spell_illusion(self, _game: &mut Game, _priority: Priority, _actor_ref: PlayerReference) -> Wizard{
        self
    }

    fn spell_reveal(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) -> Wizard{
        self
    }


    fn spell_pyrolyze(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) -> Wizard{
        self
    }


    fn spell_shock(self, game: &mut Game, priority: Priority, actor_ref: PlayerReference) -> Wizard{
        self
    }
    
}