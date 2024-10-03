
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, resolution_state::ResolutionState};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl, Role};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vigilante {
    state: VigilanteState
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum VigilanteState{
    NotLoaded,
    Loaded{bullets: u8, night_selection: <Vigilante as RoleStateImpl>::RoleActionChoice},
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

impl RoleStateImpl for Vigilante {
    type ClientRoleState = Vigilante;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority{
            Priority::TopPriority => {
                if VigilanteState::WillSuicide == self.state {
                    actor_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Suicide, AttackPower::ProtectionPiercing, false);
                    self.state = VigilanteState::Suicided;
                }
            },
            Priority::Kill => {
            
                match &self.state {
                    VigilanteState::Loaded { bullets, night_selection } if *bullets > 0 => {

                        if let Some(visit) = actor_ref.night_visits(game).first(){

                            let target_ref = visit.target;

                            let killed = target_ref.try_night_kill_single_attacker(
                                actor_ref, game,
                                GraveKiller::Role(Role::Vigilante), AttackPower::Basic, false
                            );
                            self.state = VigilanteState::Loaded { bullets: bullets.saturating_sub(1), night_selection: night_selection.clone() };

                            if killed && target_ref.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town) {
                                self.state = VigilanteState::WillSuicide;
                            }                            
                        }
                    }       

                    VigilanteState::NotLoaded => {
                        self.state = VigilanteState::Loaded {
                            bullets:3, night_selection: super::common_role::RoleActionChoiceOnePlayer::default()
                        };
                    }

                    _ => {},
                    
                }
            },
            _ => {}
        }
        actor_ref.set_role_state(game, self);
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if let VigilanteState::Loaded { bullets, .. } = self.state.clone() {
            if 
                bullets > 0 &&
                crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false)
            {
                self.state = VigilanteState::Loaded { bullets, night_selection: action_choice };
                actor_ref.set_role_state(game, self);
            }
        }
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        match self.state {
            VigilanteState::Loaded { bullets, night_selection } if bullets > 0 => {
                crate::game::role::common_role::convert_action_choice_to_visits(night_selection.player, true)
            }
            _ => Vec::new()
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        match phase {
            crate::game::phase::PhaseType::Night => {
                if let VigilanteState::Loaded { bullets, .. } = self.state {
                    actor_ref.set_role_state(game, Vigilante{
                        state: VigilanteState::Loaded { bullets, night_selection: <Self as RoleStateImpl>::RoleActionChoice::default() }
                    });
                }
            }
            _ => {}
        }
    }
}