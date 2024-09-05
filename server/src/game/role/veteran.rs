use serde::Serialize;

use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::Game;

use super::{Priority, RoleState, Role, RoleStateImpl};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Veteran { 
    alerts_remaining: u8, 
    alerting_tonight: bool 
}

impl Default for Veteran {
    fn default() -> Self {
        Veteran {
            alerts_remaining: 3,
            alerting_tonight: false
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Veteran {
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
                actor_ref.increase_defense_to(game, 2);
            }
            Priority::Kill => {
                if !self.alerting_tonight {return}

                for other_player_ref in actor_ref.all_visitors(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref
                    ).collect::<Vec<PlayerReference>>()
                {
                    other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Veteran), 2, false);
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