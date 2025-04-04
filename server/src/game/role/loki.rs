use serde::Serialize;

use crate::game::{ability_input::{AvailableIntegerSelection, ControllerID, ControllerParametersMap, IntegerSelection}, attack_power::DefensePower, event::on_midnight::OnMidnightPriority, grave::Grave, modifiers::{ModifierType, Modifiers}, phase::PhaseType, player::PlayerReference, win_condition::WinCondition, Game};


use super::{GetClientRoleState, Role, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Loki {
    next_grave_route_action: LokiGraveRouteStatus,
    next_trial_route_action: LokiTrialRouteStatus,
    next_communication_route_action: LokiCommunicationRouteStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl Loki {
    pub fn won(&self) -> bool {
        matches!(self.next_grave_route_action, LokiGraveRouteStatus::Won)
            || matches!(self.next_trial_route_action, LokiTrialRouteStatus::Won)
            || matches!(self.next_communication_route_action, LokiCommunicationRouteStatus::Won)
    }
}

impl RoleStateImpl for Loki {
    type ClientRoleState = ClientRoleState;

    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if actor_ref.night_blocked(game) { return }

        match priority {
            OnMidnightPriority::BottomPriority => {
                if let Some(IntegerSelection(action)) = game.saved_controllers.get_controller_current_selection_integer(ControllerID::Role { player: actor_ref, role: Role::Loki, id: 0 }) {
                    match LokiAction::try_from(action) {
                        Ok(action) => {
                            action.do_action(game);
                            match action {
                                LokiAction::GraveRoute(status) => {
                                    actor_ref.set_role_state(game, Self { next_grave_route_action: status.next(), ..self });
                                },
                                LokiAction::TrialRoute(status) => {
                                    actor_ref.set_role_state(game, Self { next_trial_route_action: status.next(), ..self });
                                },
                                LokiAction::CommunicationRoute(status) => {
                                    actor_ref.set_role_state(game, Self { next_communication_route_action: status.next(), ..self });
                                }
                            }
                        },
                        Err(_) => {}
                    }
                }
            },
            _ => {}
        }
    }

    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Obituary && self.won() {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }

    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::Role { player: actor_ref, role: Role::Loki, id: 0 })
            .available_selection(AvailableIntegerSelection::Discrete { values: vec![
                0, // None
                self.next_grave_route_action as i8,
                self.next_trial_route_action as i8,
                self.next_communication_route_action as i8
            ]})
            .default_selection(IntegerSelection(0))
            .reset_on_phase_start(PhaseType::Obituary)
            .night_typical(actor_ref)
            .add_grayed_out_condition(self.won())
            .build_map()
    }

    fn default_win_condition(self) -> WinCondition {
        WinCondition::RoleStateWon
    }
}

impl GetClientRoleState<ClientRoleState> for Loki {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}


#[derive(Clone, Copy)]
pub enum LokiAction {
    GraveRoute(LokiGraveRouteStatus),
    TrialRoute(LokiTrialRouteStatus),
    CommunicationRoute(LokiCommunicationRouteStatus),
}

impl TryFrom<i8> for LokiAction {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LokiAction::GraveRoute(LokiGraveRouteStatus::SkipDay1)),
            2 => Ok(LokiAction::GraveRoute(LokiGraveRouteStatus::RoleSetGraveKillers)),
            3 => Ok(LokiAction::GraveRoute(LokiGraveRouteStatus::NoDeathCause)),
            4 => Ok(LokiAction::GraveRoute(LokiGraveRouteStatus::ObscuredGraves)),
            5 => Ok(LokiAction::TrialRoute(LokiTrialRouteStatus::Abstaining)),
            6 => Ok(LokiAction::TrialRoute(LokiTrialRouteStatus::NoScheduledNominations)),
            7 => Ok(LokiAction::TrialRoute(LokiTrialRouteStatus::AutoGuilty)),
            8 => Ok(LokiAction::TrialRoute(LokiTrialRouteStatus::TwoThirdsMajority)),
            9 => Ok(LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoNightChat)),
            10 => Ok(LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::HiddenWhispers)),
            11 => Ok(LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoWhispers)),
            12 => Ok(LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoChat)),
            _ => Err(())
        }
    }
}

impl LokiAction {
    pub fn do_action(self, game: &mut Game) {
        match self {
            LokiAction::GraveRoute(LokiGraveRouteStatus::SkipDay1) => {
                Modifiers::enable_modifier(game, ModifierType::SkipDay1);
            },
            LokiAction::GraveRoute(LokiGraveRouteStatus::RoleSetGraveKillers) => {
                Modifiers::enable_modifier(game, ModifierType::RoleSetGraveKillers);
            },
            LokiAction::GraveRoute(LokiGraveRouteStatus::NoDeathCause) => {
                Modifiers::enable_modifier(game, ModifierType::NoDeathCause);
            },
            LokiAction::GraveRoute(LokiGraveRouteStatus::ObscuredGraves) => {
                Modifiers::enable_modifier(game, ModifierType::ObscuredGraves);
            },
            LokiAction::TrialRoute(LokiTrialRouteStatus::Abstaining) => {
                Modifiers::disable_modifier(game, ModifierType::NoAbstaining);
            },
            LokiAction::TrialRoute(LokiTrialRouteStatus::NoScheduledNominations) => {
                Modifiers::disable_modifier(game, ModifierType::ScheduledNominations);
            },
            LokiAction::TrialRoute(LokiTrialRouteStatus::AutoGuilty) => {
                Modifiers::enable_modifier(game, ModifierType::AutoGuilty);
            },
            LokiAction::TrialRoute(LokiTrialRouteStatus::TwoThirdsMajority) => {
                Modifiers::enable_modifier(game, ModifierType::TwoThirdsMajority);
            },
            LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoNightChat) => {
                Modifiers::enable_modifier(game, ModifierType::NoNightChat);
            },
            LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::HiddenWhispers) => {
                Modifiers::enable_modifier(game, ModifierType::HiddenWhispers);
            },
            LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoWhispers) => {
                Modifiers::enable_modifier(game, ModifierType::NoWhispers);
            },
            LokiAction::CommunicationRoute(LokiCommunicationRouteStatus::NoChat) => {
                Modifiers::enable_modifier(game, ModifierType::NoChat);
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
#[repr(i8)]
pub enum LokiGraveRouteStatus {
    #[default]
    SkipDay1 = 1,
    RoleSetGraveKillers = 2,
    NoDeathCause = 3,
    ObscuredGraves = 4,
    Won = 0
}

impl LokiGraveRouteStatus {
    pub fn next(self) -> Self {
        match self {
            LokiGraveRouteStatus::SkipDay1 => LokiGraveRouteStatus::RoleSetGraveKillers,
            LokiGraveRouteStatus::RoleSetGraveKillers => LokiGraveRouteStatus::NoDeathCause,
            LokiGraveRouteStatus::NoDeathCause => LokiGraveRouteStatus::ObscuredGraves,
            LokiGraveRouteStatus::ObscuredGraves => LokiGraveRouteStatus::Won,
            _ => self
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Default)]
#[repr(i8)]
pub enum LokiTrialRouteStatus {
    #[default]
    Abstaining = 5,
    NoScheduledNominations = 6,
    AutoGuilty = 7,
    TwoThirdsMajority = 8,
    Won = 0
}

impl LokiTrialRouteStatus {
    pub fn next(self) -> Self {
        match self {
            LokiTrialRouteStatus::Abstaining => LokiTrialRouteStatus::NoScheduledNominations,
            LokiTrialRouteStatus::NoScheduledNominations => LokiTrialRouteStatus::AutoGuilty,
            LokiTrialRouteStatus::AutoGuilty => LokiTrialRouteStatus::TwoThirdsMajority,
            LokiTrialRouteStatus::TwoThirdsMajority => LokiTrialRouteStatus::Won,
            _ => self
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
#[repr(i8)]
pub enum LokiCommunicationRouteStatus {
    #[default]
    NoNightChat = 9,
    HiddenWhispers = 10,
    NoWhispers = 11,
    NoChat = 12,
    Won = 0
}

impl LokiCommunicationRouteStatus {
    pub fn next(self) -> Self {
        match self {
            LokiCommunicationRouteStatus::NoNightChat => LokiCommunicationRouteStatus::HiddenWhispers,
            LokiCommunicationRouteStatus::HiddenWhispers => LokiCommunicationRouteStatus::NoWhispers,
            LokiCommunicationRouteStatus::NoWhispers => LokiCommunicationRouteStatus::NoChat,
            LokiCommunicationRouteStatus::NoChat => LokiCommunicationRouteStatus::Won,
            _ => self
        }
    }
}