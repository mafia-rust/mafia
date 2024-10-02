use serde::{Deserialize, Serialize};

use crate::game::attack_power::AttackPower;
use crate::game::components::poison::Poison;
use crate::game::{attack_power::DefensePower, components::puppeteer_marionette::PuppeteerMarionette};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Puppeteer{
    pub marionettes_remaining: u8,
    pub action: PuppeteerActionType,
}

impl Default for Puppeteer{
    fn default() -> Self {
        Self {
            marionettes_remaining: 3,
            action: PuppeteerActionType::Poison
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PuppeteerActionType{
    String,
    Poison,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: PuppeteerActionType
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PuppeteerAction{
    String{target: PlayerReference},
    Poison{target: PlayerReference},
    None
}

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Puppeteer {
    type ClientRoleState = Puppeteer;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Poison {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target = visit.target;
            
            match self.action {
                PuppeteerActionType::String => {
                    if AttackPower::ArmorPiercing.can_pierce(target.defense(game)) {
                        if PuppeteerMarionette::string(game, target){
                            self.marionettes_remaining -= 1;
                        }
                        actor_ref.set_role_state(game, RoleState::Puppeteer(self));
                    }else{
                        Poison::poison_player(game, 
                            target, AttackPower::ArmorPiercing, 
                            crate::game::grave::GraveKiller::Role(Role::Puppeteer), 
                            vec![actor_ref].into_iter().collect(), true
                        );
                    }
                }
                PuppeteerActionType::Poison => {
                    Poison::poison_player(game, 
                        target, AttackPower::ArmorPiercing, 
                        crate::game::grave::GraveKiller::Role(Role::Puppeteer), 
                        vec![actor_ref].into_iter().collect(), true
                    );
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
    fn create_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Night && self.marionettes_remaining == 0{
            self.action = PuppeteerActionType::Poison;
            actor_ref.set_role_state(game, RoleState::Puppeteer(self))
        }
    }
}