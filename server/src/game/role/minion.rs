use serde::Serialize;

use crate::game::{attack_power::DefensePower, grave::Grave};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{CustomClientRoleState, Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Default)]
pub struct Minion{
    currently_used_player: Option<PlayerReference> 
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for Minion {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, RoleState::Minion(Minion{
                currently_used_player: Some(currently_used_player)
            }))
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        ((
            actor_ref != target_ref &&
            actor_ref.selection(game).is_empty()
        ) || (
            actor_ref.selection(game).len() == 1
        ))
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{target: target_refs[0], attack: false}, 
                Visit{target: target_refs[1], attack: false},
            ]
        }else{
            Vec::new()
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if
            actor_ref.alive(game) &&
            !PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .any(|p|
                    p.required_resolution_states_for_win(game).is_some_and(|s1|
                        actor_ref.required_resolution_states_for_win(game).is_some_and(|s2|
                            s1.is_disjoint(&s2)
                )))

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Minion(Minion { currently_used_player: None }));
        }
    }
}

impl CustomClientRoleState<ClientRoleState> for Minion {
    fn get_client_role_state(self, _: &Game, _: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}