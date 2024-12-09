use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, GetClientRoleState, PlayerListSelection, Priority, Role, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterfeiter{
    pub fake_role: Role,
    pub fake_will: String,
    pub forges_remaining: u8,
    pub forged_ref: Option<PlayerReference>,

    pub action: CounterfeiterAction
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState{
    pub fake_role: Role,
    pub fake_will: String,
    pub forges_remaining: u8,

    pub action: CounterfeiterAction
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CounterfeiterAction{
    Forge,
    NoForge
}
impl Default for Counterfeiter {
    fn default() -> Self {
        Counterfeiter {
            forges_remaining: 3,
            forged_ref: None,
            fake_role: Role::Jester,
            fake_will: "".to_owned(),

            action: CounterfeiterAction::NoForge,
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Counterfeiter {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() <= 1 {return}

        match priority {
            Priority::Deception => {
                if self.forges_remaining == 0 || self.action == CounterfeiterAction::NoForge {return}

                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else{return};
                let target_ref = visit.target;

                target_ref.set_night_grave_role(game, Some(self.fake_role));
                target_ref.set_night_grave_will(game, self.fake_will.clone());
                actor_ref.set_role_state(game, Counterfeiter { 
                    forges_remaining: self.forges_remaining.saturating_sub(1), 
                    forged_ref: Some(target_ref), 
                    ..self
                });
            },
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
            
                    target_ref.try_night_kill_single_attacker(
                        actor_ref, game, GraveKiller::RoleSet(RoleSet::Mafia), AttackPower::Basic, false
                    );
                }
            },
            Priority::Investigative => {
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
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        game.day_number() > 1
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Counterfeiter{
            forged_ref: None,
            ..self
        });
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        let Some(PlayerListSelection(backup)) = game.saved_controllers
            .get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        )
        else {return};
        let Some(backup) = backup.first() else {return};
        if actor_ref != dead_player_ref {return}

        //convert backup to godfather
        backup.set_role_and_win_condition_and_revealed_group(game, self);
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
impl GetClientRoleState<ClientRoleState> for Counterfeiter {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            fake_role: self.fake_role,
            fake_will: self.fake_will,
            forges_remaining: self.forges_remaining,
            action: self.action,
        }
    }
}