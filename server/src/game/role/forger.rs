
use serde::{Deserialize, Serialize};

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Role};
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Forger {
    pub fake_role: Role,
    pub fake_will: String,
    pub forges_remaining: u8,
    pub forged_ref: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState{
    fake_role: Role,
    fake_will: String,
    forges_remaining: u8
}

impl Default for Forger {
    fn default() -> Self {
        Forger {
            forges_remaining: 3,
            forged_ref: None,
            fake_role: Role::Jester,
            fake_will: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: ForgerActionChoice
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ForgerActionChoice{
    SetForge{
        role: Role,
        alibi: String
    },
    SetTarget{
        target: Option<PlayerReference>
    }
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Forger {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        if self.forges_remaining == 0 {return}

        match priority {
            Priority::Deception=>{
                let Some(visit) = actor_ref.night_visits(game).first() else{return};

                let target_ref = visit.target;

                target_ref.set_night_grave_role(game, Some(self.fake_role));
                target_ref.set_night_grave_will(game, self.fake_will.clone());
                actor_ref.set_role_state(game, RoleState::Forger(Forger { 
                    forges_remaining: self.forges_remaining - 1, 
                    forged_ref: Some(target_ref), 
                    ..self
                }));
            },
            Priority::Investigative=>{
                if let Some(forged_ref) = self.forged_ref {
                    if forged_ref.night_died(game) {
                        actor_ref.push_night_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                            player: forged_ref,
                            role: forged_ref.role(game),
                            will: forged_ref.will(game).to_string(),
                        });
                    }
                }
            },
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && self.forges_remaining > 0
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Forger(Forger{
            forged_ref: None,
            ..self
        }));
    }
}
impl GetClientRoleState<ClientRoleState> for Forger {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            fake_role: self.fake_role,
            fake_will: self.fake_will.clone(),
            forges_remaining: self.forges_remaining,
        }
    }
}