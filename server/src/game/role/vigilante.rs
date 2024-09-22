
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, resolution_state::ResolutionState};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl, Role, RoleState};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vigilante {
    state: VigilanteState
}

pub type ClientRoleState = Vigilante;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum VigilanteState{
    NotLoaded,
    Loaded{bullets: u8},
    WillSuicide,
    Suicided,
}

impl Default for Vigilante {
    fn default() -> Self {
        Self { state: VigilanteState::NotLoaded }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for Vigilante {
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority{
            Priority::TopPriority => {
                if VigilanteState::WillSuicide == self.state {
                    actor_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Suicide, AttackPower::ProtectionPiercing, false);
                    self.state = VigilanteState::Suicided;
                }
            },
            Priority::Kill => {
            
                match self.state {
                    VigilanteState::Loaded { bullets } if bullets > 0 => {

                        if let Some(visit) = actor_ref.night_visits(game).first(){

                            let target_ref = visit.target;

                            let killed = target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Vigilante), AttackPower::Basic, false);
                            self.state = VigilanteState::Loaded { bullets: bullets.saturating_sub(1) };

                            if killed && ResolutionState::requires_only_this_resolution_state(game, target_ref, ResolutionState::Town) {
                                self.state = VigilanteState::WillSuicide;
                            }                            
                        }
                    }       

                    VigilanteState::NotLoaded => {
                        self.state = VigilanteState::Loaded { bullets:3 };
                    }

                    _ => {},
                    
                }
            },
            _ => {}
        }
    actor_ref.set_role_state(game, RoleState::Vigilante(self));
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && 
        if let VigilanteState::Loaded { bullets } = &self.state {
            *bullets >=1
        } else {
            false
        }
    }
    fn convert_selection_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
}