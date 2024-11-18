use serde::{Deserialize, Serialize};

use crate::{game::{
    ability_input::{
        common_selection::{
            one_player_option_selection::{AvailableOnePlayerOptionSelection, OnePlayerOptionSelection}, two_player_option_selection::{AvailableTwoPlayerOptionSelection, TwoPlayerOptionSelection}, two_role_option_selection::{AvailableTwoRoleOptionSelection, TwoRoleOptionSelection}, two_role_outline_option_selection::{AvailableTwoRoleOutlineOptionSelection, TwoRoleOutlineOptionSelection}, AvailableSelection
        },
        AbilityInput
    }, chat::ChatMessageVariant, phase::PhaseType, player::PlayerReference, Game
}, packet::ToClientPacket, vec_map::VecMap};

use super::insider_group::InsiderGroupID;




///// The client should send this over
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericAbilitySelection{
    input: VecMap<GenericAbilityID, GenericAbilitySelectionType>,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum GenericAbilitySelectionType{
    UnitSelection,
    OnePlayerOptionSelection{
        selection: OnePlayerOptionSelection
    },
    TwoPlayerOptionSelection{
        selection: TwoPlayerOptionSelection
    },
    TwoRoleOptionSelection{
        selection: TwoRoleOptionSelection
    },
    TwoRoleOutlineOptionSelection{
        selection: TwoRoleOutlineOptionSelection
    },
}


//// the server should send this over
#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct AvailableGenericAbilitySelection{
    //Indexed by generic ability ID
    //If generic ability ID is not in the map, then the ability is not available
    input: VecMap<GenericAbilityID, AvailableGenericAbilitySelectionType>,
}
impl AvailableGenericAbilitySelection{
    pub fn new(input: VecMap<GenericAbilityID, AvailableGenericAbilitySelectionType>)->Self{
        Self{input}
    }
}
impl AvailableSelection for AvailableGenericAbilitySelection{
    type Selection = GenericAbilitySelection;

    fn validate_selection(&self, selection: &Self::Selection)->bool {
        selection.input.iter().all(|(id, selection)|{
            self.input.get(id).map_or(false, |available_selection_type|
                available_selection_type.validate_selection(selection)
            )
        })
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum AvailableGenericAbilitySelectionType{
    UnitSelection,
    OnePlayerOptionSelection{selection: AvailableOnePlayerOptionSelection},
    TwoPlayerOptionSelection{selection: AvailableTwoPlayerOptionSelection},
    TwoRoleOptionSelection{selection: AvailableTwoRoleOptionSelection},
    TwoRoleOutlineOptionSelection{selection: AvailableTwoRoleOutlineOptionSelection},
}

impl AvailableSelection for AvailableGenericAbilitySelectionType{
    type Selection = GenericAbilitySelectionType;

    fn validate_selection(&self, selection: &Self::Selection)->bool {
        match (self, selection){
            (
                AvailableGenericAbilitySelectionType::UnitSelection,
                GenericAbilitySelectionType::UnitSelection
            ) => true,
            (
                AvailableGenericAbilitySelectionType::OnePlayerOptionSelection{selection: available},
                GenericAbilitySelectionType::OnePlayerOptionSelection{selection}
            ) => available.validate_selection(selection),
            (
                AvailableGenericAbilitySelectionType::TwoPlayerOptionSelection { selection: available },
                GenericAbilitySelectionType::TwoPlayerOptionSelection{selection}
            ) => available.validate_selection(selection),
            (
                AvailableGenericAbilitySelectionType::TwoRoleOptionSelection{selection: available},
                GenericAbilitySelectionType::TwoRoleOptionSelection{selection}
            ) => available.validate_selection(selection),
            (
                AvailableGenericAbilitySelectionType::TwoRoleOutlineOptionSelection{selection: available},
                GenericAbilitySelectionType::TwoRoleOutlineOptionSelection{selection}
            ) => available.validate_selection(selection),
            _ => false
        }
    }
}



pub type GenericAbilityID = u8;
type PlayerSavedInput = (AvailableGenericAbilitySelection, VecMap<GenericAbilityID, GenericAbilitySelectionType>);
#[derive(Default)]
pub struct GenericAbilitySaveComponent{
    players_saved_inputs: VecMap<PlayerReference, PlayerSavedInput>
}
impl GenericAbilitySaveComponent{
    pub fn on_ability_input_received(
        game: &mut Game,
        actor_ref: PlayerReference,
        ability_input: AbilityInput
    ){
        let AbilityInput::GenericAbility { selection } = ability_input else {return};

        //if there is no saved input for the player, create one
        if !game
            .generic_ability
            .players_saved_inputs
            .contains(&actor_ref)
        {
            game.generic_ability.players_saved_inputs.insert(
                actor_ref,
                (
                    AvailableGenericAbilitySelection::default(),
                    VecMap::default()
                )
            );
        }

        //get the saved input, it was just created so it should exist
        let Some(saved_input_for_player) = game
            .generic_ability
            .players_saved_inputs
            .get_mut(&actor_ref) else {return};

        //validate selection
        if !saved_input_for_player.0.validate_selection(&selection){return;}

        //messages to send saying who you selected, need this vec due to borrow checker stuff
        let mut selection_message_queue = Vec::new();

        //save selection, if the player has already saved a selection for this ability, update it
        for (id, selection) in selection.input.clone(){
            if let Some(ability_data) = saved_input_for_player.1.get_mut(&id) {
                if ability_data != &selection{
                    selection_message_queue.push((actor_ref, id, selection.clone()));
                    *ability_data = selection;
                }
            }else{
                selection_message_queue.push((actor_ref, id, selection.clone()));
                saved_input_for_player.1.insert(id, selection);
            };
        }

        actor_ref.send_packet(game, ToClientPacket::GenericAbilitySelection{
            selection: selection.clone()
        });

        for (actor_ref, id, selection) in selection_message_queue{
            Self::send_selection_message(game, actor_ref, id, selection);
        }
    }


    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        if phase != PhaseType::Obituary {return}
        game.generic_ability.players_saved_inputs.clear();
        for player in PlayerReference::all_players(game){
            player.send_packet(game, ToClientPacket::ClearGenericAbilitySelection);
        }
    }

    pub fn on_tick(game: &mut Game){
        for player in PlayerReference::all_players(game){
            let new_available_selection = 
                player.role_state(game).clone().available_generic_ability_selection(game, player);
            
            let current = Self::current_available_generic_ability_selection(game, player);

            if
                current.is_none() ||
                current.is_some_and(|c| c != new_available_selection)
            {
                Self::set_available_generic_ability_selection(game, player, new_available_selection);
            }
        }
    }


    fn set_available_generic_ability_selection(
        game: &mut Game,
        player_ref: PlayerReference,
        available_selection: AvailableGenericAbilitySelection
    ){
        if let Some(saved_input) = game.generic_ability.players_saved_inputs.get_mut(&player_ref){
            saved_input.0 = available_selection;
        }else{
            game.generic_ability.players_saved_inputs.insert(
                player_ref,
                (available_selection, VecMap::default())
            );
        }

        let Some(available_selection) = 
            Self::current_available_generic_ability_selection(game, player_ref) else {return};

        player_ref.send_packet(game, ToClientPacket::AvailableGenericAbilitySelection{
            available_selection: available_selection.clone()
        });
    }

    fn current_available_generic_ability_selection(
        game: &Game,
        player_ref: PlayerReference
    )->Option<AvailableGenericAbilitySelection>{
        game.generic_ability.players_saved_inputs
            .get(&player_ref)
            .map(|data| data.0.clone())
    }

    pub fn get_all_saved_input(game: &Game, player_ref: PlayerReference)->Option<VecMap<GenericAbilityID, GenericAbilitySelectionType>>{
        game.generic_ability.players_saved_inputs
            .get(&player_ref)
            .map(|data| data.1.clone())
    }

    pub fn get_saved_input(game: &Game, player_ref: PlayerReference, id: GenericAbilityID)->Option<GenericAbilitySelectionType>{
        game.generic_ability.players_saved_inputs
            .get(&player_ref)
            .and_then(|data| data.1.get(&id).map(|x| x.clone()))
    }


    pub fn send_selection_message(
        game: &mut Game,
        player_ref: PlayerReference,
        id: GenericAbilityID,
        selection: GenericAbilitySelectionType
    ){
        let chat_message = ChatMessageVariant::GenericAbilityUsed{
            player: player_ref.index(),
            role: Some(player_ref.role(game)),
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
}

