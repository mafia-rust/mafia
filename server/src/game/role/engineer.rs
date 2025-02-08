use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::night_visits::NightVisits;
use crate::game::grave::GraveKiller;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{common_role, BooleanSelection, ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Default, Clone, Debug)]
pub struct Engineer {
    pub trap: Trap
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    trap: ClientTrapState
}

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum ClientTrapState {
    Dismantled,
    Ready,
    Set
}

#[derive(Default, Clone, Debug)]
pub enum Trap {
    #[default]
    Dismantled,
    Ready,
    Set{target: PlayerReference}
}

impl Trap {
    fn state(&self) -> TrapState {
        match self {
            Trap::Dismantled => TrapState::Dismantled,
            Trap::Ready => TrapState::Ready,
            Trap::Set{..} => TrapState::Set
        }
    }
}
#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TrapState {
    #[default]
    Dismantled,
    Ready,
    Set
}

//engineer prioritys
//tell player state

//Set trap / ready up / choose to unset and bring to ready
//protect, kill & investigate, dismantle



pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Engineer {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Heal => {
                //upgrade state

                if !actor_ref.night_blocked(game) {
                    match self.trap {
                        Trap::Dismantled => {
                            actor_ref.set_role_state(game, Engineer {trap: Trap::Ready});
                        },
                        Trap::Ready => {
                            if let Some(visit) = actor_ref.untagged_night_visits_cloned(game).first(){
                                actor_ref.set_role_state(game, Engineer {trap: Trap::Set{target: visit.target}});
                            }
                        },
                        Trap::Set { .. } => {
                            if let Some(BooleanSelection(true)) = game.saved_controllers.get_controller_current_selection_boolean(
                                ControllerID::role(actor_ref, Role::Engineer, 1)
                            ){
                                actor_ref.set_role_state(game, Engineer {trap: Trap::Ready});
                            }
                        }
                    }
                }
    
                if let RoleState::Engineer(Engineer{trap: Trap::Set{target, ..}}) = actor_ref.role_state(game).clone(){
                    target.increase_defense_to(game, DefensePower::Protection);
                }
            }
            Priority::Kill => {
                if let Trap::Set { target, .. } = self.trap {
                    for visit in NightVisits::all_visits(game).into_iter().cloned().collect::<Vec<_>>() {
                        if 
                            visit.attack &&
                            visit.target == target &&
                            visit.visitor != actor_ref
                        {
                            visit.visitor.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Engineer), AttackPower::ArmorPiercing, false);
                        }
                    }
                }
            }
            Priority::Investigative => {
                if let Trap::Set { target, .. } = self.trap {

                    let mut should_dismantle = false;

                    if target.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target.push_night_message(game, ChatMessageVariant::YouWereProtected);
                        should_dismantle = true;
                    }

                    for visitor in target.all_night_visitors_cloned(game) {
                        if visitor != actor_ref{
                            actor_ref.push_night_message(game, ChatMessageVariant::EngineerVisitorsRole { role: visitor.role(game) });
                            should_dismantle = true;
                        }
                    }

                    if should_dismantle {
                        actor_ref.set_role_state(game, RoleState::Engineer(Engineer {trap: Trap::Dismantled}));
                    }
                }

                actor_ref.push_night_message(game, ChatMessageVariant::TrapStateEndOfNight { state: self.trap.state() });
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        match self.trap {
            Trap::Ready => {
                common_role::controller_parameters_map_player_list_night_typical(
                    game,
                    actor_ref,
                    false,
                    true,
                    false,
                    ControllerID::role(actor_ref, Role::Engineer, 0)
                )
            },
            Trap::Set { .. } => {
                common_role::controller_parameters_map_boolean(
                    game,
                    actor_ref,
                    false,
                    ControllerID::role(actor_ref, Role::Engineer, 1)
                )
            }
            _ => {
                ControllerParametersMap::default()
            }
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref, 
            ControllerID::role(actor_ref, Role::Engineer, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                actor_ref.add_private_chat_message(game, ChatMessageVariant::TrapState { state: self.trap.state() });
            }
            _ => {}
        }
    }

}
impl GetClientRoleState<ClientRoleState> for Engineer {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            trap: match self.trap {
                Trap::Dismantled => ClientTrapState::Dismantled,
                Trap::Ready => ClientTrapState::Ready,
                Trap::Set {..} => ClientTrapState::Set,
            }
        }
    }
}