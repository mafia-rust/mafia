use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::confused::Confused;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::grave::GraveKiller;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize)]
pub struct Vigiloop {
    state: VigiloopState
}

#[derive(Clone, Debug, Serialize)]
enum VigiloopState {
    NotLoaded,
    Loaded {bullets: u8},
}

impl Default for Vigiloop {
    fn default() -> Self {
        Self { state: VigiloopState::NotLoaded }
    }
}

impl RoleStateImpl for Vigiloop {
    type ClientRoleState = Vigiloop;
    fn on_midnight(mut self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if priority != OnMidnightPriority::Kill {return;}

        match self.state {
            VigiloopState::Loaded { bullets } if bullets > 0 => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(midnight_variables);
                
                if let Some(visit) = actor_visits.first(){
                    if 
                        Confused::is_confused(game, actor_ref) ||
                        !actor_ref.all_night_visitors_cloned(midnight_variables).is_empty()
                    {
                        actor_ref.push_night_message(midnight_variables, 
                            ChatMessageVariant::SomeoneSurvivedYourAttack
                        );
                    } else {
                        visit.target.try_night_kill_single_attacker(
                            actor_ref,
                            game,
                            midnight_variables,
                            GraveKiller::Role(Role::Vigiloop),
                            AttackPower::Basic,
                            false
                        );
                    }
                    self.state = VigiloopState::Loaded { bullets: bullets.saturating_sub(1) };
                }
            }
            VigiloopState::NotLoaded => {
                self.state = VigiloopState::Loaded { bullets: game.num_players().div_ceil(5) };
            }
            _ => {}
        }

        actor_ref.set_role_state(game, self);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let can_shoot = if let VigiloopState::Loaded { bullets } = &self.state {
            *bullets >=1
        } else {
            false
        };

        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Vigiloop, 0))
            .single_player_selection_typical(actor_ref, false, true)
            .night_typical(actor_ref)
            .add_grayed_out_condition(!can_shoot)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Vigiloop, 0),
            false
        )
    }
}