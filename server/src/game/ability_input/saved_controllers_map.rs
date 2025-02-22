use serde::{Deserialize, Serialize};

use crate::{
    game::{
        chat::ChatMessageVariant, components::{
            forfeit_vote::ForfeitVote, insider_group::InsiderGroupID, mafia::Mafia, pitchfork::Pitchfork, syndicate_gun_item::SyndicateGunItem
        }, 
        event::{
            on_controller_selection_changed::OnControllerSelectionChanged,
            on_validated_ability_input_received::OnValidatedAbilityInputReceived
        }, 
        phase::PhaseType, player::PlayerReference, Game
    }, packet::ToClientPacket, vec_map::VecMap, vec_set::VecSet
};

use super::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedControllersMap{
    saved_controllers: VecMap<ControllerID, SavedController>,
}

impl SavedControllersMap{
    pub fn new(saved_controllers: VecMap<ControllerID, SavedController>)->Self{
        Self{saved_controllers}
    }

    //event listeners
    /// returns false if any of these
    /// - selection is invalid
    /// - the controllerId doesnt exist
    /// - the controller is grayed out
    /// - actor is not allowed for this controller
    /// - the incoming selection is the same as the current selection
    pub fn on_ability_input_received(
        game: &mut Game,
        actor: PlayerReference,
        ability_input: AbilityInput
    )->bool{
        let AbilityInput{
            id, selection: incoming_selection
        } = ability_input.clone();

        if !Self::set_selection_in_controller(game, actor, id.clone(), incoming_selection.clone(), false) {
            return false;
        }

        OnValidatedAbilityInputReceived::new(actor, ability_input).invoke(game);

        Self::send_selection_message(game, actor, id, incoming_selection);
        
        Self::send_saved_controllers_to_clients(game);

        true
    }

    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        Self::update_controllers_from_parameters(game);
        for (_, saved_controller) in game.saved_controllers.saved_controllers.iter_mut(){
            saved_controller.reset_on_phase_start(phase);
        }
        Self::send_saved_controllers_to_clients(game);
    }

    pub fn on_tick(game: &mut Game){
        Self::update_controllers_from_parameters(game);
    }


    // mutators
    fn update_controllers_from_parameters(game: &mut Game){
        let mut new_controller_parameters_map = ControllerParametersMap::default();

        for player in PlayerReference::all_players(game) {
            new_controller_parameters_map.combine_overwrite(player.controller_parameters_map(game));
        }

        new_controller_parameters_map.combine_overwrite(
            SyndicateGunItem::controller_parameters_map(game)
        );
        new_controller_parameters_map.combine_overwrite(
            Mafia::controller_parameters_map(game)
        );
        new_controller_parameters_map.combine_overwrite(
            ForfeitVote::controller_parameters_map(game)
        );
        new_controller_parameters_map.combine_overwrite(
            Pitchfork::controller_parameters_map(game)
        );

        let current_controller_parameters = &game.saved_controllers.controller_parameters();

        if *current_controller_parameters != new_controller_parameters_map {
            Self::set_controller_parameters(game, new_controller_parameters_map);
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
    
    /// Keeps old selection if its valid, otherwise uses default_selection,
    /// even if default selection is invalid
    fn set_controller_parameters(game: &mut Game, new_controller_parameters_map: ControllerParametersMap){

        let controller_ids_to_remove = game.saved_controllers.controller_parameters().controller_parameters().keys()
            .filter(|id| !new_controller_parameters_map.controller_parameters().contains_key(id))
            .cloned()
            .collect::<Vec<_>>();

        for id in controller_ids_to_remove{
            game.saved_controllers.saved_controllers.remove(&id);
        }

        for (id, controller_parameters) in new_controller_parameters_map.controller_parameters().iter(){
            let mut new_selection = controller_parameters.default_selection().clone();
            
            let mut kept_old_selection = false;
            

            if let Some(SavedController{selection: old_selection, ..}) = game.saved_controllers.saved_controllers.get(&id) {
                if 
                    controller_parameters.validate_selection(game, old_selection) &&
                    !controller_parameters.dont_save() &&
                    !controller_parameters.grayed_out()
                {
                    new_selection = old_selection.clone();
                    kept_old_selection = true;
                }
            }
            

            game.saved_controllers.saved_controllers.insert(
                id.clone(),
                SavedController::new(
                    new_selection,
                    controller_parameters.clone()
                )
            );

            if !kept_old_selection {
                OnControllerSelectionChanged::new(id.clone()).invoke(game);
            }
        }

        Self::send_saved_controllers_to_clients(game);
    }

    /// return true if selection was valid
    /// return false if selection was invalid (ie wrong actor)
    /// if selection is invalid then nothing happens, nothing is updated
    pub fn set_selection_in_controller(
        game: &mut Game,
        actor: PlayerReference,
        id: ControllerID,
        selection: AbilitySelection,
        overwrite_gray_out: bool
    )->bool{
        Self::update_controllers_from_parameters(game);

        // validate input using available selection
        {
            let Some(SavedController {
                selection: saved_selection,
                available_ability_data
            }) = game.saved_controllers.saved_controllers.get(&id) else {return false};
            
            if 
                !available_ability_data.validate_selection(game, &selection) ||
                (!overwrite_gray_out && available_ability_data.grayed_out()) ||
                !available_ability_data.allowed_players().contains(&actor) ||
                (*saved_selection == selection && selection != AbilitySelection::new_unit())
            {
                return false;
            }
        }
        

        let Some(SavedController {
            selection: saved_selection,
            available_ability_data
        }) = game.saved_controllers.saved_controllers.get_mut(&id) else {return false};

        if !available_ability_data.dont_save() {
            *saved_selection = selection.clone();
            OnControllerSelectionChanged::new(id).invoke(game);
        }

        true
    }

    // new query
    pub fn all_controllers(&self)->&VecMap<ControllerID, SavedController>{
        &self.saved_controllers
    }
    pub fn all_controller_ids(&self)->VecSet<ControllerID>{
        self.saved_controllers.iter()
            .map(|(c, _)|c.clone())
            .collect()
    }

    pub fn controllers_allowed_to_player(&self, player: PlayerReference)->SavedControllersMap{
        SavedControllersMap::new(
            self.saved_controllers.iter()
                .filter(|(_, saved_controller)| saved_controller.available_ability_data.allowed_players().contains(&player))
                .map(|(id, saved_controller)| (id.clone(), saved_controller.clone()))
                .collect()
        )
    }
    
    pub fn controller_parameters(&self)->ControllerParametersMap{
        ControllerParametersMap::new(
            self.saved_controllers.iter()
                .map(|(id, saved_controller)| (id.clone(), saved_controller.available_ability_data.clone()))
                .collect()
        )
    }
    
    pub fn controller_parameters_allowed_to_player(&self, player: PlayerReference)->ControllerParametersMap{
        ControllerParametersMap::new(
            self.controller_parameters().controller_parameters().iter()
                .filter(|(_, saved_controller)| saved_controller.allowed_players().contains(&player))
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect()
        )
    }

    pub fn get_controller(&self, id: ControllerID)->Option<&SavedController>{
        self.saved_controllers.get(&id)
    }

    pub fn get_controller_current_selection(&self, id: ControllerID)->Option<AbilitySelection>{
        self
            .get_controller(id)
            .map(|saved_controller| saved_controller.selection.clone())
    }

    // selection type queries

    pub fn get_controller_current_selection_boolean(&self,id: ControllerID)->Option<BooleanSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::Boolean { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_player_list(&self,id: ControllerID)->Option<PlayerListSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::PlayerList { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_two_player_option(&self,id: ControllerID)->Option<TwoPlayerOptionSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::TwoPlayerOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_role_option(&self,id: ControllerID)->Option<RoleOptionSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::RoleOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_two_role_option(&self,id: ControllerID)->Option<TwoRoleOptionSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::TwoRoleOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_two_role_outline_option(&self,id: ControllerID)->Option<TwoRoleOutlineOptionSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::TwoRoleOutlineOption { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_string(&self,id: ControllerID)->Option<StringSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::String { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }

    pub fn get_controller_current_selection_integer(&self,id: ControllerID)->Option<IntegerSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::Integer { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
    
    pub fn get_controller_current_selection_kira(&self,id: ControllerID)->Option<KiraSelection>{
        self
            .get_controller_current_selection(id)
            .and_then(|selection| 
                if let AbilitySelection::Kira { selection } = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
    
    
    
    // game stuff
    
    pub fn send_saved_controllers_to_clients(game: &Game){
        for player in PlayerReference::all_players(game){
            player.send_packet(game, ToClientPacket::YourAllowedControllers { 
                save: game.saved_controllers.controllers_allowed_to_player(player).saved_controllers
            });
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedController{
    selection: AbilitySelection,

    available_ability_data: ControllerParameters
}
impl SavedController{
    fn new(selection: AbilitySelection, available_ability_data: ControllerParameters)->Self{
        Self{selection, available_ability_data}
    }
    pub fn selection(&self)->&AbilitySelection{
        &self.selection
    }
    pub fn reset_on_phase_start(&mut self, phase: PhaseType){
        if let Some(reset_phase) = self.available_ability_data.reset_on_phase_start(){
            if phase == reset_phase{
                self.selection = self.available_ability_data.default_selection().clone();
            }
        }
    }
}