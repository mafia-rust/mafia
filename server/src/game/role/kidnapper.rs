use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::detained::Detained;
use crate::game::grave::{Grave, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role::BooleanSelection;
use crate::game::visit::Visit;
use crate::game::win_condition::WinCondition;
use crate::game::Game;
use crate::vec_set;

use super::{
    AbilitySelection, AvailableAbilitySelection, ControllerID,
    ControllerParametersMap, PlayerListSelection, Priority, Role,
    RoleStateImpl
};


#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kidnapper { 
    pub jailed_target_ref: Option<PlayerReference>, 
    executions_remaining: u8
}

impl Default for Kidnapper {
    fn default() -> Self {
        Self { 
            jailed_target_ref: None, 
            executions_remaining: 1
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Kidnapper {
    type ClientRoleState = Kidnapper;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {


        match priority {
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first() {
    
                    let target_ref = visit.target;
                    if Detained::is_detained(game, target_ref){
                        target_ref.try_night_kill_single_attacker(
                            actor_ref, 
                            game, 
                            GraveKiller::Role(Role::Jailor), 
                            AttackPower::ProtectionPiercing, 
                            false
                        );
        
                        self.executions_remaining = self.executions_remaining.saturating_sub(1);
                        actor_ref.set_role_state(game, self);
                    }
                }
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Kidnapper, 0),
            AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|target_ref|
                        target_ref.alive(game) &&
                        actor_ref != *target_ref
                    )
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game),
            Some(PhaseType::Night),
            false,
            vec_set!(actor_ref)
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Kidnapper, 1),
                AvailableAbilitySelection::new_boolean(),
                AbilitySelection::new_boolean(false),
                actor_ref.ability_deactivated_from_death(game) ||
                Detained::is_detained(game, actor_ref) || 
                self.executions_remaining <= 0 ||
                game.day_number() <= 1 ||
                self.jailed_target_ref.is_none(),
                Some(PhaseType::Obituary),
                false,
                vec_set!(actor_ref)
            )
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let Some(AbilitySelection::Boolean {selection: BooleanSelection(true)}) = game.saved_controllers.get_controller_current_selection(
            ControllerID::role(actor_ref, Role::Kidnapper, 1)) else {return Vec::new()};
        let Some(target) = self.jailed_target_ref else {return Vec::new()};
        return vec![Visit::new_none(actor_ref, target, true)]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, 
            if PlayerReference::all_players(game).any(|p|Detained::is_detained(game, p)) {
                vec![ChatGroup::Kidnapped].into_iter().collect()
            }else{
                vec![]
            }
        )
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);
        if 
            game.current_phase().is_night() &&
            !actor_ref.ability_deactivated_from_death(game) &&
            PlayerReference::all_players(game).any(|p|Detained::is_detained(game, p))
        {
            out.insert(ChatGroup::Kidnapped);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Kidnapper, 0)
                ) else {return};
                let Some(target) = target.first() else {return};

                if actor_ref.ability_deactivated_from_death(game) || !target.alive(game) {return};
                
                self.jailed_target_ref = Some(*target);
                
                actor_ref.set_role_state(game, self);

                Detained::add_detain(game, *target);
                actor_ref.add_private_chat_message(game, 
                    ChatMessageVariant::JailedTarget{ player_index: target.index() }
                );
            },
            PhaseType::Obituary => {
                self.jailed_target_ref = None;
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }

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