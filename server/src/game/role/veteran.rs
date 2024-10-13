use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;

use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

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
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                if self.alerts_remaining > 0 && game.day_number() > 1{
                    if let Some(selection) = actor_ref.selection(game).first(){
                        if *selection == actor_ref{
                            actor_ref.set_role_state(game, RoleState::Veteran(Veteran { 
                                alerts_remaining: self.alerts_remaining - 1, 
                                alerting_tonight: true 
                            }));
                        }
                    }
                }
            }
            Priority::Heal=>{
                if !self.alerting_tonight {return}
                actor_ref.increase_defense_to(game, DefensePower::Protection);
            }
            Priority::Kill => {
                if !self.alerting_tonight {return}

                for other_player_ref in actor_ref.all_visitors(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref
                    ).collect::<Vec<PlayerReference>>()
                {
                    other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Veteran), AttackPower::ArmorPiercing, false);
                }
            }
            _=>{}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        self.alerts_remaining > 0 &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        game.day_number() > 1
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Veteran(Veteran { alerts_remaining: self.alerts_remaining, alerting_tonight: false }));   
    }
}
impl GetClientRoleState<ClientRoleState> for Veteran {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            alerts_remaining: self.alerts_remaining
        }
    }
}