use serde::Serialize;

use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::common_role::RoleActionChoiceTwoPlayers;
use super::{GetClientRoleState, Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct Retributionist { 
    used_bodies: Vec<PlayerReference>, 
    currently_used_player: Option<PlayerReference>,
    night_selection: super::common_role::RoleActionChoiceTwoPlayers
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState{
    night_selection: super::common_role::RoleActionChoiceTwoPlayers
}

impl RoleStateImpl for Retributionist {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceTwoPlayers;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            let mut used_bodies = self.used_bodies;
            used_bodies.push(currently_used_player);

            actor_ref.set_role_state(game, Retributionist{
                used_bodies,
                currently_used_player: Some(currently_used_player),
                ..self
            })
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};

        self.night_selection = match action_choice.two_players {
            Some((a, b)) => {
                if 
                    !actor_ref.night_jailed(game) &&
                    actor_ref.alive(game) &&
                    !a.alive(game) &&
                    b.alive(game) &&
                    self.used_bodies.iter().filter(|p| **p == a).count() < 2 &&
                    game.graves.iter().any(|grave|
                        grave.player == a && 
                        if let Some(role) = grave.role(){
                            role.faction() == Faction::Town
                        }else{false}
                    )
                {
                    action_choice
                }else{
                    super::common_role::RoleActionChoiceTwoPlayers{two_players: None}
                }
            },
            _ => action_choice
        };

        actor_ref.set_role_state(game, self)
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        super::common_role::convert_action_choice_to_visits_two_players(&self.night_selection, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Obituary {
            actor_ref.set_role_state(game, Retributionist { used_bodies: self.used_bodies, currently_used_player: None, night_selection: RoleActionChoiceTwoPlayers{two_players: None} });
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Retributionist {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            night_selection: self.night_selection
        }
    }
}