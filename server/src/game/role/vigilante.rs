
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, Role, RoleState};

pub(super) const SUSPICIOUS: bool = false;
pub(super) const WITCHABLE: bool = true;
pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vigilante {
    bullets_remaining: u8,
    will_suicide: bool,
}
impl Default for Vigilante {
    fn default() -> Self {
        Self { bullets_remaining: 3, will_suicide: false }
    }
}
impl RoleStateImpl for Vigilante {
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        match priority{
            Priority::TopPriority => {
                if self.will_suicide {
                    actor_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 3);
                }
            },
            Priority::Kill => {
                if self.bullets_remaining == 0 || self.will_suicide || game.phase_machine.day_number == 1 {return;}

                if let Some(visit) = actor_ref.night_visits(game).first(){
                    self.bullets_remaining -= 1;

                    let target_ref = visit.target;
                    if target_ref.night_jailed(game){
                        actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                        return
                    }

                    let killed = target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Vigilante), 1);

                    if killed && target_ref.role(game).faction_alignment().faction() == Faction::Town {
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
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref) && self.bullets_remaining > 0 && !self.will_suicide && game.phase_machine.day_number != 1
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self,  _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}