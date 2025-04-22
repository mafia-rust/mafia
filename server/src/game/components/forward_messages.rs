use crate::game::{ability_input::{AbilityInput, AbilitySelection, AvailableChatMessageSelection, ControllerID, ControllerParametersMap}, chat::ChatMessageVariant, phase::PhaseState, player::PlayerReference, Game};

use super::insider_group::InsiderGroupID;

pub struct ForwardMessages;

impl ForwardMessages{
    pub fn on_validated_ability_input_received(game: &mut Game, actor: PlayerReference, input: AbilityInput){
        let (
            ControllerID::ForwardMessage{player},
            AbilitySelection::ChatMessage(selection)
        ) = input.id_and_selection() else {return};
        if actor != player {return}
        let Some(message) = selection.0 else {return};
        let message = Box::new(message.variant().clone());

        InsiderGroupID::send_message_in_available_insider_chat_or_private(
            game,
            actor,
            ChatMessageVariant::PlayerForwardedMessage{message, forwarder: actor},
            false,
        )
    }
    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap{
        ControllerParametersMap::combine(
            PlayerReference::all_players(game)
                .filter(|p| InsiderGroupID::in_any_group(game, *p))
                .map(|player|
                    ControllerParametersMap::builder(game)
                        .id(ControllerID::ForwardMessage { player })
                        .available_selection(AvailableChatMessageSelection)
                        .add_grayed_out_condition(!matches!(game.current_phase(), PhaseState::Night | PhaseState::Obituary {..}))
                        .dont_save()
                        .allow_players([player])
                        .build_map()
                )
        )
    }
}