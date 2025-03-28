use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::Game;
use crate::vec_set;
use super::{
    AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap,
    PlayerListSelection, Priority, Role, RoleStateImpl
};

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
    pub interviewed_target: Option<PlayerReference>, 
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Reporter {
    type ClientRoleState = Reporter;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if 
            priority == Priority::Investigative &&
            Self::get_public(game, actor_ref) && 
            !actor_ref.ability_deactivated_from_death(game) &&
            !actor_ref.night_blocked(game) &&
            !actor_ref.night_silenced(game)
        {
            game.add_message_to_chat_group(
                ChatGroup::All, 
                ChatMessageVariant::ReporterReport { report: Self::get_report(game, actor_ref)}
            );    
        }
    }
    
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Reporter, 0),
            AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|p|p.alive(game) && *p != actor_ref)
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game),
            Some(PhaseType::Night),
            false,
            vec_set![actor_ref]
        ).combine_overwrite_owned(
            //publicize
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Reporter, 1),
                AvailableAbilitySelection::new_boolean(),
                AbilitySelection::new_boolean(false),
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                vec_set![actor_ref]
            )
        ).combine_overwrite_owned(
            //report
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Reporter, 2),
                AvailableAbilitySelection::new_string(),
                AbilitySelection::new_string(String::new()),
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                vec_set![actor_ref]
            )
        )
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, 
            if 
                game.current_phase().is_night() &&
                !actor_ref.ability_deactivated_from_death(game) &&
                self.interviewed_target.map_or_else(||false, |p|p.alive(game))
            {
                vec![ChatGroup::Interview]
            }else{
                vec![]
            }
        )
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);
        if 
            game.current_phase().is_night() &&
            !actor_ref.ability_deactivated_from_death(game) &&
            self.interviewed_target.map_or_else(||false, |p|p.alive(game))
        {
            out.insert(ChatGroup::Interview);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Reporter, 0)
                ) else {return};
                let Some(target) = target.first() else {return};

                if actor_ref.ability_deactivated_from_death(game) || !target.alive(game) {return};
                
                self.interviewed_target = Some(*target);
                
                actor_ref.set_role_state(game, self);

                InsiderGroupID::send_message_in_available_insider_chat_or_private(
                    game,
                    *target,
                    ChatMessageVariant::PlayerIsBeingInterviewed { player_index: target.index() },
                    true
                );
            },
            PhaseType::Obituary => {
                self.interviewed_target = None;
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}

impl Reporter{
    fn get_report(game: &Game, actor_ref: PlayerReference)->String{
        game.saved_controllers.get_controller_current_selection_string(
            ControllerID::role(actor_ref, Role::Reporter, 2)
        ).map_or_else(String::new, |s|s.0)
    }
    fn get_public(game: &Game, actor_ref: PlayerReference)->bool{
        game.saved_controllers.get_controller_current_selection_boolean(
            ControllerID::role(actor_ref, Role::Reporter, 1)
        ).is_some_and(|b|b.0)
    }
}