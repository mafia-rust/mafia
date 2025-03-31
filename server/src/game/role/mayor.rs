
use serde::Serialize;

use crate::game::ability_input::AvailableUnitSelection;
use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::event::on_whisper::{OnWhisper, WhisperFold, WhisperPriority};
use crate::game::modifiers::Modifiers;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::tag::Tag;
use crate::game::Game;
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
        game.count_nomination_and_start_trial(
            !Modifiers::modifier_is_enabled(game, crate::game::modifiers::ModifierType::ScheduledNominations)
        );
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _new: super::RoleState, _old: super::RoleState) {
        if actor_ref != player {return;}
        for player in PlayerReference::all_players(game){
            player.remove_player_tag(game, actor_ref, Tag::Enfranchised);
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Mayor, 0))
            .available_selection(AvailableUnitSelection)
            .add_grayed_out_condition(
                actor_ref.ability_deactivated_from_death(game) ||
                self.revealed || 
                PhaseType::Night == game.current_phase().phase() ||
                PhaseType::Briefing == game.current_phase().phase()
            )
            .dont_save()
            .allow_players([actor_ref])
            .build_map()
    }
    fn on_whisper(self, _game: &mut Game, actor_ref: PlayerReference, event: &OnWhisper, fold: &mut WhisperFold, priority: WhisperPriority) {
        if priority == WhisperPriority::Cancel && (
            event.sender == actor_ref || 
            event.receiver == actor_ref
        ) {
            fold.cancelled = true;
            fold.hide_broadcast = true;
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Mayor {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}