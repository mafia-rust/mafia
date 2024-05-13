
use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::chat::ChatGroup;
use crate::game::grave::GraveReference;
use crate::game::grave::GraveRole;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::Role;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Forger {
    pub fake_role: Option<Role>,
    pub fake_will: String,
    pub forges_remaining: u8,
    pub forged_ref: Option<PlayerReference>
}

impl Default for Forger {
    fn default() -> Self {
        Forger {
            forges_remaining: 2,
            forged_ref: None,
            fake_role: None,
            fake_will: "".to_owned(),
        }
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Forger {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        if self.forges_remaining == 0 {return}

        match priority {
            Priority::Deception=>{
                let Some(visit) = actor_ref.night_visits(game).first() else{return};

                let target_ref = visit.target;

                target_ref.set_night_grave_role(game, Some(
                    GraveRole::Role(if let Some(role) = self.fake_role {role} else {target_ref.role(game)})
                ));
                target_ref.set_night_grave_will(game, if !self.fake_will.is_empty() {self.fake_will.clone()} else {target_ref.will(game).to_owned()});
                actor_ref.set_role_state(game, RoleState::Forger(Forger { 
                    forges_remaining: self.forges_remaining - 1, 
                    forged_ref: Some(target_ref), 
                    ..self
                }));
            },
            Priority::Investigative=>{
                if let Some(forged_ref) = self.forged_ref {
                    if forged_ref.night_died(game) {
                        actor_ref.push_night_message(game, ChatMessageVariant::PlayerRoleAndWill{
                            role: forged_ref.role(game),
                            will: forged_ref.will(game).to_string(),
                        });
                    }
                }
            },
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && self.forges_remaining > 0
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Forger(Forger{
            forged_ref: None,
            ..self
        }));
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
