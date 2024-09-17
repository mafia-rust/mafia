use serde::{Deserialize, Serialize};

use crate::game::components::puppeteer_marionette::PuppeteerMarionette;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, RoleStateImpl};

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

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 1;

impl RoleStateImpl for Puppeteer {
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target = visit.target;
            
            match self.action {
                PuppeteerAction::String => {
                    if target.defense(game) < 2 {
                        PuppeteerMarionette::string(game, target);
                        self.marionettes_remaining -= 1;
                        actor_ref.set_role_state(game, RoleState::Puppeteer(self));
                    }else{
                        PuppeteerMarionette::poison(game, target);
                    }
                }
                PuppeteerAction::Poison => {
                    PuppeteerMarionette::poison(game, target);
                },
            }
        }

        
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        !PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Night && self.marionettes_remaining == 0{
            self.action = PuppeteerAction::Poison;
            actor_ref.set_role_state(game, RoleState::Puppeteer(self))
        }
    }
}