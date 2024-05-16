use serde::{Deserialize, Serialize};

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::puppeteer_marionette::PuppeteerMarionette;
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Puppeteer{
    pub marionettes_remaining: u8,
    pub action: PuppeteerAction,
}
impl Default for Puppeteer{
    fn default() -> Self {
        Self {
            marionettes_remaining: 3,
            action: PuppeteerAction::Poison
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PuppeteerAction{
    String,
    Poison
}

impl RoleStateImpl for Puppeteer {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target = visit.target;
            
            match self.action {
                PuppeteerAction::String => {
                    PuppeteerMarionette::zombify(game, target);
                    actor_ref.push_night_message(game, ChatMessageVariant::PuppeteerPlayerIsNowMarionette{player: target.index()});
                    target.insert_role_label(game, actor_ref);
                    actor_ref.insert_role_label(game, target);
                }
                PuppeteerAction::Poison => {
                    PuppeteerMarionette::poison(game, target);
                },
            }
        }

        
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
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
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Night {
            self.action = PuppeteerAction::Poison;
            actor_ref.set_role_state(game, RoleState::Puppeteer(self))
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}