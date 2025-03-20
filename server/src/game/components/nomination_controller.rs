use crate::{
    game::{
        ability_input::{AbilityInput, AvailablePlayerListSelection, ControllerID, ControllerParametersMap, PlayerListSelection}, chat::{ChatGroup, ChatMessageVariant}, modifiers::{ModifierType, Modifiers}, player::PlayerReference, Game
    }, packet::ToClientPacket
};

pub struct NominationController;

impl NominationController{
    pub fn controller_parameters_map(game: &mut Game)->ControllerParametersMap{
        PlayerReference::all_players(game)
            .map(|actor| Self::one_player_controller(game, actor))
            .fold(ControllerParametersMap::default(), |mut acc, controller| {
                acc.combine_overwrite(controller);
                acc
            })
    }
    fn one_player_controller(game: &mut Game, actor: PlayerReference)->ControllerParametersMap{
        ControllerParametersMap::builder()
            .id(crate::game::ability_input::ControllerID::Nominate { player: actor })
            .available_selection(game, AvailablePlayerListSelection {
                available_players: PlayerReference::all_players(game).filter(|p|p.alive(game)).collect(),
                can_choose_duplicates: false,
                max_players: Some(1)
            })
            .add_grayed_out_condition(
                !actor.alive(game) ||
                actor.forfeit_vote(game) ||
                game.current_phase().phase() != crate::game::phase::PhaseType::Nomination
            )
            .reset_on_phase_start(crate::game::phase::PhaseType::Nomination)
            .allowed_players([actor])
            .build_map(game)
    }
    pub fn on_validated_ability_input_received(game: &mut Game, player: PlayerReference, input: AbilityInput){
        if let Some(PlayerListSelection(voted)) = input.get_player_list_selection_if_id(ControllerID::Nominate{ player }){

            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::Voted{
                voter: player.index(), 
                votee: voted.first().map(|p|p.index())
            });

            game.count_nomination_and_start_trial(
                !Modifiers::modifier_is_enabled(game, ModifierType::ScheduledNominations)
            );

            let packet = ToClientPacket::new_player_votes(game);
            game.send_packet_to_all(packet);
        }
    }
}