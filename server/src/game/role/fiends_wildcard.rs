use serde::{Serialize, Deserialize};

use crate::game::{attack_power::DefensePower, role_list::RoleSet};
use crate::game::chat::ChatMessageVariant;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::role_can_generate;
use crate::game::Game;
use crate::vec_set;

use super::{AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap, Role, RoleOptionSelection, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FiendsWildcard;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for FiendsWildcard {
    type ClientRoleState = FiendsWildcard;
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                if actor_ref.ability_deactivated_from_death(game) {return;}
                self.become_role(game, actor_ref);
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::FiendsWildcard, 0),
            AvailableAbilitySelection::new_role_option(
                RoleSet::Fiends.get_roles().into_iter().filter(|role|
                    game.settings.enabled_roles.contains(role) && *role != Role::FiendsWildcard
                ).map(|r|Some(r)).chain(std::iter::once(None)).collect()
            ),
            AbilitySelection::new_role_option(None),
            actor_ref.ability_deactivated_from_death(game),
            None,
            false,
            vec_set!(actor_ref)
        )
    }
}

impl FiendsWildcard {
    fn become_role(&self, game: &mut Game, actor_ref: PlayerReference) {

        let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
            ControllerID::role(actor_ref, Role::FiendsWildcard, 0)
        ) else {return};

        println!("role: {:?}", role);

        if
            role_can_generate(
                role, 
                &game.settings.enabled_roles, 
                &PlayerReference::all_players(game)
                    .map(|player_ref| player_ref.role(game))
                    .collect::<Vec<Role>>()
            )
        {
            actor_ref.set_role_and_win_condition_and_revealed_group(game, role.new_state(game));
        }else{
            actor_ref.add_private_chat_message(game, ChatMessageVariant::WildcardConvertFailed{role})
        }
    }
}