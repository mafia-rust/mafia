use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::visit::Visit;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::Game;

use super::{Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
pub struct Veteran { 
    alerts_remaining: u8, 
    night_selection: super::common_role::RoleActionChoiceBool
}

impl Default for Veteran {
    fn default() -> Self {
        Veteran {
            alerts_remaining: 3,
            night_selection: super::common_role::RoleActionChoiceBool::default()
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Veteran {
    type ClientRoleState = Self;
    type RoleActionChoice = super::common_role::RoleActionChoiceBool;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                if self.alerts_remaining > 0 && game.day_number() > 1 && self.night_selection.boolean{
                    actor_ref.set_role_state(game, Veteran { 
                        alerts_remaining: self.alerts_remaining - 1, 
                        ..self
                    });
                }
            }
            Priority::Heal=>{
                if !self.night_selection.boolean {return}
                actor_ref.increase_defense_to(game, DefensePower::Protection);
            }
            Priority::Kill => {
                if !self.night_selection.boolean {return}

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
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_boolean_is_valid(game, actor_ref) {return}
        if self.alerts_remaining <= 0 || game.day_number() <= 1 {return}
        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        vec![]
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Veteran {
            alerts_remaining: self.alerts_remaining,
            night_selection: <Self as RoleStateImpl>::RoleActionChoice::default()
        });   
    }
}