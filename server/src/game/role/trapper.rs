use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleState, RoleStateImpl};

#[derive(Default, Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trapper {
    trap: Trap
}
#[derive(Default, Clone, Serialize, Debug)]
pub enum Trap {
    #[default]
    Dismantled,
    Set{target: PlayerReference}
}
impl Trap {
    fn is_dismantled(&self) -> bool {
        matches!(self, Trap::Dismantled)
    }
    fn state(&self) -> TrapState {
        match self {
            Trap::Dismantled => TrapState::Dismantled,
            Trap::Set{..} => TrapState::Set
        }
    }
}
#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TrapState {
    #[default]
    Dismantled,
    Set
}

//trapper prioritys

//protect, kill & investigate
//Set trap


pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Trapper {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Heal => {
                if let Trap::Set { target } = self.trap {
                    target.increase_defense_to(game, 2);
                }
            }
            Priority::Kill => {
                if let Trap::Set { target } = self.trap {
                    for attacker in PlayerReference::all_players(game) {
                        if attacker.night_visits(game).iter().any(|visit| visit.target == target && visit.attack){
                            attacker.try_night_kill(actor_ref, game, crate::game::grave::GraveKiller::Role(Role::Trapper), 2, false);
                            actor_ref.push_night_message(game, ChatMessageVariant::TrapperYouAttackedVisitor);
                        }
                    }
                }
            }
            Priority::Investigative => {
                if let Trap::Set { target } = self.trap {

                    if target.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }

                    for visitor in PlayerReference::all_players(game) {
                        if visitor.night_visits(game).iter().any(|visit|visit.target == target){
                            actor_ref.push_night_message(game, ChatMessageVariant::TrapperVisitorsRole { role: visitor.role(game) });
                        }
                    }
                }
            }
            Priority::FinalPriority => {

                let mut caught_role = false;
                if let Trap::Set { target } = self.trap {
                    for visitor in PlayerReference::all_players(game) {
                        if visitor.night_visits(game).iter().any(|visit|visit.target == target){
                            caught_role = true;
                            break;
                        }
                    }
                }
                

                //if trap just triggered or manual dismantle, then dismantle
                if
                    caught_role ||
                    actor_ref.night_visits(game).iter().any(|visit| visit.target == actor_ref)
                {
                    actor_ref.set_role_state(game, RoleState::Trapper(Trapper {trap: Trap::Dismantled}));
                }else
                //set trap
                if let Some(visit) = actor_ref.night_visits(game).first(){
                    
                    if self.trap.is_dismantled(){
                        actor_ref.set_role_state(game, RoleState::Trapper(Trapper {trap: Trap::Set{target: visit.target}}));
                    }
                }
                
            }
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        (match self.trap {
            Trap::Set { target } => actor_ref == target,
            Trap::Dismantled => actor_ref != target_ref,
        }) &&
        !actor_ref.night_jailed(game) &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false)
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
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                if actor_ref.alive(game) {
                    actor_ref.add_private_chat_message(game, ChatMessageVariant::TrapState { state: self.trap.state() });
                }
            }
            _ => {}
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}