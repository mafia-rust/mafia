use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::godfather::Godfather;
use super::{
    AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap, GetClientRoleState, IntegerSelection, Priority, Role, RoleOptionSelection, RoleStateImpl, StringSelection
};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterfeiter{
    pub forges_remaining: u8,
    pub forged_ref: Option<PlayerReference>
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState{
    pub forges_remaining: u8
}
impl Default for Counterfeiter {
    fn default() -> Self {
        Counterfeiter {
            forges_remaining: 3,
            forged_ref: None
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Counterfeiter {
    type ClientRoleState = ClientRoleState;
    fn new_state(game: &Game) -> Self {
        Self{
            forges_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() <= 1 {return}

        match priority {
            Priority::Deception => {
                if self.forges_remaining == 0 || chose_no_forge(game, actor_ref) {return}
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else{return};

                let target_ref = visit.target;

                let fake_role = if let Some(RoleOptionSelection(fake_role)) = game.saved_controllers
                    .get_controller_current_selection_role_option(ControllerID::role(actor_ref, Role::Counterfeiter, 1)) {
                    fake_role
                } else {
                    None
                };

                target_ref.set_night_grave_role(game, fake_role);

                let fake_alibi = if let Some(StringSelection(string)) = game.saved_controllers
                    .get_controller_current_selection_string(ControllerID::role(actor_ref, Role::Counterfeiter, 2)) {
                    string
                } else {
                    "".to_owned()
                };
                target_ref.set_night_grave_will(game, fake_alibi);

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
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Counterfeiter, 0),
            true
        )
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Counterfeiter, 0)
        ).combine_overwrite_owned(
            //role
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Counterfeiter, 1),
                AvailableAbilitySelection::new_role_option(
                    Role::values().into_iter()
                        .map(|role| Some(role))
                        .collect()
                ),
                AbilitySelection::new_role_option(Some(Role::Counterfeiter)),
                self.forges_remaining == 0 ||
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                vec_set![actor_ref]
            )
        ).combine_overwrite_owned(
            //alibi
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Counterfeiter, 2),
                AvailableAbilitySelection::new_string(),
                AbilitySelection::new_string(String::new()),
                self.forges_remaining == 0 ||
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                vec_set![actor_ref]
            )
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Counterfeiter, 3),
                super::AvailableAbilitySelection::new_integer(0, 
                    if self.forges_remaining > 0 {1} else {0}
                ),
                AbilitySelection::new_integer(0),
                false,
                None,
                false,
                vec_set!(actor_ref),
            )
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, Counterfeiter{
            forged_ref: None,
            ..self
        });
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        Godfather::pass_role_state_down(game, actor_ref, dead_player_ref, self);
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
            forges_remaining: self.forges_remaining
        }
    }
}

fn chose_no_forge(game: &Game, actor_ref: PlayerReference)->bool{
    if let Some(IntegerSelection(x)) = game.saved_controllers.get_controller_current_selection_integer(
        ControllerID::role(actor_ref, Role::Counterfeiter, 3)
    ){
        x == 0
    }else{
        true
    }
}