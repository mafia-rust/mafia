use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::{common_role, BooleanSelection, ControllerID, GetClientRoleState, Priority, Role, RoleStateImpl};

#[derive(Debug, Clone)]
pub struct Veteran { 
    alerts_remaining: u8, 
    alerting_tonight: bool 
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    alerts_remaining: u8
}

impl Default for Veteran {
    fn default() -> Self {
        Veteran {
            alerts_remaining: 3,
            alerting_tonight: false
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Veteran {
    type ClientRoleState = ClientRoleState;
    fn new_state(game: &Game) -> Self {
        Self{
            alerts_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                let can_alert = self.alerts_remaining > 0 && game.day_number() > 1;
                let chose_to_alert = 
                    matches!(game.saved_controllers.get_controller_current_selection_boolean(
                        ControllerID::role(actor_ref, Role::Veteran, 0)
                    ), Some(BooleanSelection(true)));

                if can_alert && chose_to_alert{
                    actor_ref.set_role_state(game, Veteran { 
                        alerts_remaining: self.alerts_remaining.saturating_sub(1), 
                        alerting_tonight: true 
                    });
                }
            }
            Priority::Heal=>{
                if !self.alerting_tonight {return}
                actor_ref.increase_defense_to(game, DefensePower::Protection);
            }
            Priority::Kill => {
                if !self.alerting_tonight {return}

                for other_player_ref in actor_ref.all_night_visitors_cloned(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref
                    ).collect::<Vec<PlayerReference>>()
                {
                    other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Veteran), AttackPower::ArmorPiercing, false, false);
                }
            }
            _=>{}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        common_role::controller_parameters_map_boolean(
            game,
            actor_ref,
            self.alerts_remaining == 0 || game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Veteran, 0)
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(
            game,
            Veteran { alerts_remaining: self.alerts_remaining, alerting_tonight: false });   
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}
impl GetClientRoleState<ClientRoleState> for Veteran {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            alerts_remaining: self.alerts_remaining
        }
    }
}