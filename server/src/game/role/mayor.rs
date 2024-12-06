
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::tag::Tag;
use crate::game::Game;
use crate::vec_set;
use super::{ControllerID, ControllerParametersMap, GetClientRoleState, Role, RoleStateImpl};

#[derive(Clone, Debug, Default)]
pub struct Mayor {
    pub revealed: bool
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Mayor {
    type ClientRoleState = ClientRoleState;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: super::AbilityInput) {
        if actor_ref != input_player {return;}
        if ability_input.id() != ControllerID::role(actor_ref, Role::Mayor, 0) {
            return;
        }
        

        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MayorRevealed { player_index: actor_ref.index() });

        actor_ref.set_role_state(game, Mayor{
            revealed: true
        });
        for player in PlayerReference::all_players(game){
            player.push_player_tag(game, actor_ref, Tag::Enfranchised);
        }
        game.count_votes_and_start_trial();
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Mayor, 0),
            super::AvailableAbilitySelection::Unit,
            super::AbilitySelection::new_unit(),
            !actor_ref.alive(game) ||
            self.revealed || 
            PhaseType::Night == game.current_phase().phase() ||
            PhaseType::Briefing == game.current_phase().phase(),
            None,
            true,
            vec_set![actor_ref]
        )
    }
}
impl GetClientRoleState<ClientRoleState> for Mayor {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}