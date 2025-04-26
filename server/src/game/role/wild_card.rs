use serde::{Serialize, Deserialize};

use crate::game::ability_input::{AbilityInput, AvailableRoleOptionSelection};
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::player::PlayerReference;
use crate::game::role_list::role_can_generate;
use crate::game::Game;

use super::{ControllerID, ControllerParametersMap, Role, RoleOptionSelection, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Wildcard;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Wildcard {
    type ClientRoleState = Wildcard;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, _input_player: PlayerReference, ability_input: AbilityInput) {
        let Some(RoleOptionSelection(Some(role))) = ability_input.get_role_option_selection_if_id(ControllerID::role(
            actor_ref, 
            Role::Wildcard, 
            0
        )) else {return};
        Self::become_role(game, actor_ref, role);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Wildcard, 0))
            .available_selection(AvailableRoleOptionSelection(
                Role::values().into_iter().filter(|role|
                    game.settings.enabled_roles.contains(role) && *role != Role::Wildcard
                ).map(Some).chain(std::iter::once(None)).collect()
            ))
            .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game))
            .allow_players([actor_ref])
            .build_map()
    }
}

impl Wildcard {
    pub fn become_role(game: &mut Game, actor_ref: PlayerReference, role: Role) {
        let Some(RoleOptionSelection(Some(role))) = ControllerID::role(actor_ref, role, 0)
            .get_role_option_selection(game)
            .cloned()
            else {return};

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