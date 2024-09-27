
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Steward {
    self_heals_remaining: u8,
    target_healed_refs: Vec<PlayerReference>,
    pub role_chosen: Option<Role>,
    previous_role_chosen: Option<Role>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    steward_protects_remaining: u8,
    role_chosen: Option<Role>,
    previous_role_chosen: Option<Role>
}

impl Default for Steward {
    fn default() -> Self {
        Self { 
            self_heals_remaining: 1,
            target_healed_refs: vec![],
            role_chosen: None,
            previous_role_chosen: None
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Steward {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceRole;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                actor_ref.set_role_state(game, RoleState::Steward(
                    Steward {
                        self_heals_remaining: self.self_heals_remaining, 
                        target_healed_refs: vec![],
                        role_chosen: self.role_chosen,
                        previous_role_chosen: self.previous_role_chosen
                    }
                ));
            }
            Priority::Heal => {
                let mut healed_players = vec![];
                let mut healed_role = self.role_chosen;

                if healed_role == Some(Role::Steward) && self.self_heals_remaining == 0 {healed_role=None}
                if healed_role == self.previous_role_chosen {healed_role=None}

                if let Some(role) = healed_role {
                    for player in PlayerReference::all_players(game){
                        if role != player.role(game) {continue;}
    
                        player.increase_defense_to(game, DefensePower::Protection);
                        healed_players.push(player);
                    }
                }
                
                let self_heals_remaining = if healed_role == Some(Role::Steward) {self.self_heals_remaining.saturating_sub(1)}else{self.self_heals_remaining};
                actor_ref.set_role_state(game, RoleState::Steward(Steward{
                    self_heals_remaining,
                    target_healed_refs: healed_players,
                    role_chosen: healed_role,
                    previous_role_chosen: healed_role, //updates here
                }));
            }
            Priority::Investigative => {
                for target_healed_ref in self.target_healed_refs{
                    if target_healed_ref.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_healed_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Steward(Steward{
            self_heals_remaining: self.self_heals_remaining,
            target_healed_refs: vec![],
            role_chosen: None,
            previous_role_chosen: self.previous_role_chosen
        }));
    }
}
impl GetClientRoleState<ClientRoleState> for Steward {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            steward_protects_remaining: self.self_heals_remaining,
            role_chosen: self.role_chosen,
            previous_role_chosen: self.previous_role_chosen
        }
    }
}