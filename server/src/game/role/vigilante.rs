
use serde::Serialize;

use crate::game::{chat::ChatGroup, game_over_state::GameOverState};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
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

impl RoleStateImpl for Vigilante {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    

    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
    
        match priority{
            Priority::TopPriority => {
                if VigilanteState::WillSuicide == self.state {
                    actor_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 3, false);
                    self.state = VigilanteState::Suicided;
                }
            },
            Priority::Kill => {
            
                match self.state {
                    VigilanteState::Loaded { bullets } if bullets > 0 => {

                        if let Some(visit) = actor_ref.night_visits(game).first(){

                            let target_ref = visit.target;

                            let killed = target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Vigilante), 1, false);
                            self.state = VigilanteState::Loaded { bullets: bullets.saturating_sub(1) };

                            if killed && GameOverState::exclusively_wins_with(game, target_ref, GameOverState::Town) {
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
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && 
        if let VigilanteState::Loaded { bullets } = &self.state {
            *bullets >=1
        } else {
            false
        }
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self,  _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self,  _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave: crate::game::grave::GraveReference) {
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}