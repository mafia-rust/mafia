use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::grave::Grave;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleState, RoleStateImpl};
use rand::prelude::SliceRandom;


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scarecrow {
    pub target_ref: Option<PlayerReference>,
    pub blocked_players: Vec<PlayerReference>,
}



pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Scarecrow {
    type ClientRoleState = Scarecrow;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Ward => {
                if let Some(visit) = actor_ref.untagged_night_visits_cloned(game).first() {
                    let blocked_players = visit.target.ward(game);
                    actor_ref.set_role_state(game, RoleState::Scarecrow(Scarecrow {
                        target_ref: Some(visit.target), 
                        blocked_players,
                    }));
                } else {
                    actor_ref.set_role_state(game, RoleState::Scarecrow(Scarecrow {
                        target_ref: None, 
                        blocked_players: Vec::new(),
                    }));
                }
                
            }
            Priority::Investigative => {
                let Some(target_ref) = self.target_ref else {return};
                
                if Confused::is_confused(game, actor_ref) || self.blocked_players.is_empty() {
                    actor_ref.push_night_message(game, ChatMessageVariant::ScarecrowResult{players: Vec::new()});
                    return;
                }

                let mut blocked_players = self.blocked_players.clone();
                blocked_players.shuffle(&mut rand::rng());

                let message = ChatMessageVariant::ScarecrowResult { players:
                    PlayerReference::ref_vec_to_index(self.blocked_players.as_slice())
                };

                for player_ref in self.blocked_players.iter(){
                    actor_ref.insert_role_label(game, *player_ref);
                }
                actor_ref.insert_role_label(game, target_ref);
                
                actor_ref.push_night_message(game, message);
            },
            _=>(),
        }
        
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Scarecrow, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Scarecrow, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    WinCondition::are_friends(&p.win_condition(game), actor_ref.win_condition(game))
                )

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
}