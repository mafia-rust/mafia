
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Doctor {
    self_heals_remaining: u8,
    target_healed_ref: Option<PlayerReference>,
    night_selection: <Self as RoleStateImpl>::RoleActionChoice
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    self_heals_remaining: u8,
    night_selection: <Doctor as RoleStateImpl>::RoleActionChoice
}

impl Default for Doctor {
    fn default() -> Self {
        Self { 
            self_heals_remaining: 1,
            target_healed_ref: None,
            night_selection: <Self as RoleStateImpl>::RoleActionChoice::default()
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Doctor {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Heal => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                target_ref.increase_defense_to(game, DefensePower::Protection);

                if actor_ref == target_ref{
                    actor_ref.set_role_state(game, Doctor{
                        self_heals_remaining: self.self_heals_remaining.saturating_sub(1), 
                        target_healed_ref: Some(target_ref),
                        ..self
                    });
                }else{
                    actor_ref.set_role_state(game, Doctor{
                        target_healed_ref: Some(target_ref),
                        ..self
                    });
                }

            }
            Priority::Investigative => {
                if let Some(target_healed_ref) = self.target_healed_ref {
                    if target_healed_ref.night_attacked(game){
                        
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_healed_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, self.self_heals_remaining > 0){
            return
        }
        if game.day_number() == 1 {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(self.night_selection.player, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Doctor{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_ref: None,
            night_selection: <Self as RoleStateImpl>::RoleActionChoice::default()
        });
    }
}impl GetClientRoleState<ClientRoleState> for Doctor {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            self_heals_remaining: self.self_heals_remaining,
            night_selection: self.night_selection
        }
    }
}