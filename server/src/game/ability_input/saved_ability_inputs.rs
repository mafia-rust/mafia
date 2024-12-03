use serde::{Deserialize, Serialize};

use crate::{
    game::{
        chat::ChatMessageVariant, 
        components::{
            forfeit_vote::ForfeitVote, insider_group::InsiderGroupID,
            pitchfork::Pitchfork, syndicate_gun_item::SyndicateGunItem
        },
        phase::PhaseType, player::PlayerReference, Game
    },
    packet::ToClientPacket,
    vec_map::VecMap
};

use super::*;

#[derive(Default)]
pub struct SavedControllers{
    saved_controllers: VecMap<ControllerID, SingleSavedController>,
}

impl SavedControllers{
    //event listeners
    pub fn on_ability_input_received(
        game: &mut Game,
        actor: PlayerReference,
        ability_input: AbilityInput
    ){
        let (id, incoming_selection) = (ability_input.id, ability_input.selection);

        // validate input using available selection
        {
            let Some(SingleSavedController {
                selection: saved_selection,
                available_ability_data
            }) = game.saved_controllers.saved_controllers.get(&id) else {return};
            
            if 
                !available_ability_data.validate_selection(game, &incoming_selection) ||
                available_ability_data.grayed_out() ||
                !available_ability_data.allowed_players().contains(&actor) ||
                *saved_selection == incoming_selection
            {
                return;
            }
        }
        

        let Some(SingleSavedController {
            selection: saved_selection,
            available_ability_data
        }) = game.saved_controllers.saved_controllers.get_mut(&id) else {return};

        if !available_ability_data.dont_save() {
            *saved_selection = incoming_selection.clone();
        }

        Self::send_selection_message(game, actor, id, incoming_selection);
        Self::send_saved_abilities(game, actor);
    }



    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        for (_, saved_controller) in game.saved_controllers.saved_controllers.iter_mut(){
            saved_controller.reset_on_phase_start(phase);
        }
        for player in PlayerReference::all_players(game){
            Self::send_saved_abilities(game, player);
        }
    }

    pub fn on_tick(game: &mut Game){
        let mut all_players_available_abilities = ControllerParametersMap::default();

        for player in PlayerReference::all_players(game) {
            all_players_available_abilities.combine_overwrite(player.available_abilities(game));
        }

        all_players_available_abilities.combine_overwrite(
            SyndicateGunItem::available_abilities(game)
        );
        all_players_available_abilities.combine_overwrite(
            ForfeitVote::available_abilities(game)
        );
        all_players_available_abilities.combine_overwrite(
            Pitchfork::available_abilities(game)
        );

        for player in PlayerReference::all_players(game){
            let current = Self::get_available_abilities_data(game, player);

            let new_available = all_players_available_abilities.players().get(&player).map_or_else(
                || None,
                |new_available| if current != *new_available {
                    Some(new_available)
                }else{None}
            );

            if let Some(new_available) = new_available {
                Self::set_player_available_abilities(game, player, new_available.clone());
            }
        }
    }


    //mutators
    /// Keeps old selection if its valid, otherwise uses default_selection,
    /// even if default selection is invalid
    fn set_player_available_abilities(
        game: &mut Game,
        player: PlayerReference,
        new_available_ability_data: ControllerParametersMap
    ){
        //set new available, try to keep old selection if possible
        
        game.saved_controllers.players_saved_inputs.insert(player, 
            if let Some(current_saved_input) = game.saved_controllers.players_saved_inputs.get(&player){

                let mut new_saved_input_map: VecMap<ControllerID, SingleSavedController> = VecMap::new();

                for (ability_id, new_single_ability_data) in new_available_ability_data.abilities().clone().into_iter() {
                    
                    let mut new_selection = new_single_ability_data.default_selection().clone();

                    //if its grayed out or don't save, it should reset to the default here
                    if !new_single_ability_data.dont_save() && !new_single_ability_data.grayed_out(){
                        if let Some(SingleSavedController{selection: old_selection, ..}) = current_saved_input.save.get(&ability_id) {
                            if new_single_ability_data.validate_selection(game, old_selection){
                                new_selection = old_selection.clone()
                            }
                        }
                    };
                    
                    new_saved_input_map.insert(ability_id.clone(), SingleSavedController::new(
                        new_selection,
                        new_single_ability_data
                    ));
                }
                PlayerSavedAbilities::new(new_saved_input_map)
            }else{
                PlayerSavedAbilities::new(
                    new_available_ability_data.abilities()
                        .iter()
                        .map(|(id, available_data)|{

                            let selection = available_data.default_selection().clone();
                            let available_data = available_data.clone();
                            (id.clone(), SingleSavedController::new(selection, available_data))

                        })
                        .collect()
                )
            }
        );

        //inform client
        Self::send_saved_abilities(game, player);
    }

    fn send_saved_abilities(game: &Game, player: PlayerReference){
        if let Some(player_saved_input) = game.saved_controllers.players_saved_inputs.get(&player) {
            player.send_packet(game, ToClientPacket::YourSavedAbilities { 
                save: player_saved_input.clone()
            });
        }
    }

    pub fn send_selection_message(
        game: &mut Game,
        player_ref: PlayerReference,
        id: ControllerID,
        selection: AbilitySelection
    ){
        let chat_message = ChatMessageVariant::AbilityUsed{
            player: player_ref.index(),
            ability_id: id,
            selection: selection.clone()
        };

        let mut target_message_sent = false;
        for insider_group in InsiderGroupID::all_insider_groups_with_player(game, player_ref){
            game.add_message_to_chat_group( insider_group.get_insider_chat_group(), chat_message.clone());
            target_message_sent = true;
        }
        if !target_message_sent{
            player_ref.add_private_chat_message(game, chat_message);
        }
    }

    // query
    pub fn get_player_saved_abilities(
        game: &Game,
        player_ref: PlayerReference
    )->PlayerSavedAbilities{
        game.saved_controllers.players_saved_inputs
            .get(&player_ref).cloned()
            .unwrap_or_default()
    }

    pub fn get_available_abilities_data(game: &Game, player_ref: PlayerReference)->ControllerParametersMap{
        ControllerParametersMap::new(
            Self::get_player_saved_abilities(game, player_ref).save.into_iter()
                .map(|(id, saved_single_ability)| 
                    (id, saved_single_ability.available_ability_data))
                .collect()
        )
    }


    pub fn get_saved_ability_selection(
        game: &Game,
        player_ref: PlayerReference,
        id: ControllerID
    )->Option<AbilitySelection>{
        game.saved_controllers.players_saved_inputs
            .get(&player_ref)
            .and_then(|data| data.save.get(&id))
            .map(|save_input| save_input.selection.clone())
    }

    pub fn get_role_option_selection_if_id(
        game: &Game,
        player_ref: PlayerReference,
        id: ControllerID
    )->Option<RoleOptionSelection>{
        Self::get_saved_ability_selection(game, player_ref, id)
            .and_then(|selection| 
                if let AbilitySelection::RoleOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
    pub fn get_two_role_outline_option_selection_if_id(
        game: &Game,
        player_ref: PlayerReference,
        id: ControllerID
    )->Option<TwoRoleOutlineOptionSelection>{
        Self::get_saved_ability_selection(game, player_ref, id)
            .and_then(|selection| 
                if let AbilitySelection::TwoRoleOutlineOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SingleSavedController{
    selection: AbilitySelection,

    available_ability_data: ControllerParameters
}
impl SingleSavedController{
    fn new(selection: AbilitySelection, available_ability_data: ControllerParameters)->Self{
        Self{selection, available_ability_data}
    }
    pub fn reset_on_phase_start(&mut self, phase: PhaseType){
        if let Some(reset_phase) = self.available_ability_data.reset_on_phase_start(){
            if phase == reset_phase{
                self.selection = self.available_ability_data.default_selection().clone();
            }
        }
    }
}