use serde::Serialize;

use crate::game::grave::Grave;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, phase::PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{GetClientRoleState, Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Default)]
pub struct MafiaWitch{
    currently_used_player: Option<PlayerReference>,
    night_selection: <Self as RoleStateImpl>::RoleActionChoice
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState{
    night_selection: super::common_role::RoleActionChoiceTwoPlayers
}

impl RoleStateImpl for MafiaWitch {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceTwoPlayers;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, MafiaWitch{
                currently_used_player: Some(currently_used_player),
                ..self
            })
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};

        self.night_selection = if 
            super::common_role::default_action_choice_two_players_is_valid(game, actor_ref, action_choice.two_players, (false, true), true)
        {
            action_choice
        }else{
            super::common_role::RoleActionChoiceTwoPlayers{two_players: None}
        };

        actor_ref.set_role_state(game, self)
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        super::common_role::convert_action_choice_to_visits_two_players(self.night_selection.two_players, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    WinCondition::can_win_together(&p.win_condition(game), actor_ref.win_condition(game))
                )

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
        if phase == PhaseType::Obituary {
            actor_ref.set_role_state(game, MafiaWitch { currently_used_player: None, night_selection: super::common_role::RoleActionChoiceTwoPlayers{two_players: None} });
        }
    }
}
impl GetClientRoleState<ClientRoleState> for MafiaWitch {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            night_selection: self.night_selection
        }
    }
}