use std::collections::HashSet;

use serde::Serialize;

use crate::game::ability_input::*;
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatGroup;
use crate::game::components::detained::Detained;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};

use super::{
    Role, RoleStateImpl
};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Psychopomp{
    pub targets: VecSet<PlayerReference>,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Psychopomp {
    type ClientRoleState = Psychopomp;
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Psychopomp, 0),
            AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|target_ref|
                        !target_ref.alive(game) &&
                        actor_ref != *target_ref
                    )
                    .collect(),
                false,
                Some(2)
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game),
            Some(PhaseType::Night),
            false,
            vec_set!(actor_ref)
        )
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Dead]);

        if 
            (game.current_phase().phase() == PhaseType::Obituary) &&
            actor_ref.alive(game)
        {
            out.insert(ChatGroup::Dead);
        }
        out
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);

        if
            (
                (
                    !Detained::is_detained(game, actor_ref) &&
                    game.current_phase().phase() == PhaseType::Night
                ) || 
                game.current_phase().phase() == PhaseType::Obituary
            ) &&
            actor_ref.alive(game)
        {
            out.insert(ChatGroup::Dead);
        }
        out
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                //reset old
                self.targets = VecSet::new();
                actor_ref.set_role_state(game, self.clone());
                
                //set new
                let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Psychopomp, 0)
                ) else {return};
                
                if actor_ref.ability_deactivated_from_death(game) {return};
                
                self.targets = target.into_iter().collect();
                
                actor_ref.set_role_state(game, self);
            },
            _=>{}
        }
    }
}
impl Psychopomp {

}