use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::None;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Wizard{
    level: u8
}

impl Default for Vigilante {
    fn default() -> Self {
        Self { level: 0 }
    }
}

impl RoleStateImpl for Wizard {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        match priority{
            Priority::Kill => {
                if self.bullets_remaining == 0 || self.will_suicide || game.phase_machine.day_number == 1 {return;}

                if let Some(visit) = actor_ref.night_visits(game).first(){
                    self.bullets_remaining -= 1;

                    let target_ref = visit.target;
                    if target_ref.night_jailed(game){
                        actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                        return
                    }

                    let killed = target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Vigilante), 1, false);

                    if killed && target_ref.role(game).faction() == Faction::Town {
                        self.will_suicide = true;
                    }

                    actor_ref.set_role_state(game, RoleState::Vigilante(self));
                }
            },
            _ => {}
        }
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
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
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}

impl Wizard {
    pub fn player_is_suspicious(game: &Game, player_ref: PlayerReference) -> bool {

        if player_ref.has_suspicious_aura(game){
            true
        }else if player_ref.has_innocent_aura(game){
            false
        }else{
            player_ref.role(game).faction() != Faction::Town
        }
    }
}