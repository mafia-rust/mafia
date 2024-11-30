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
    vec_map::VecMap};

use super::{
    ability_selection::AbilitySelection,
    available_abilities_data::{available_single_ability_data::AvailableSingleAbilityData, AvailableAbilitiesData},
    selection_type::two_role_outline_option_selection::TwoRoleOutlineOptionSelection, AbilityID,
    AbilityInput
};

#[derive(Default)]
pub struct AllPlayersSavedAbilityInputs{
    players_saved_inputs: VecMap<PlayerReference, PlayerSavedAbilities>
}

impl AllPlayersSavedAbilityInputs{
    //event listeners
    pub fn on_ability_input_received(
        game: &mut Game,
        actor: PlayerReference,
        ability_input: AbilityInput
    ){
        let (id, incoming_selection) = (ability_input.id, ability_input.selection);

        // validate input using available selection
        {
            let Some(saved_ability_input) = 
            game.saved_ability_inputs.players_saved_inputs.get(&actor) else {return};
            

            let Some(SavedSingleAbility{
                selection: saved_selection,
                available_ability_data,
            }) = 
                saved_ability_input.save.get(&id).clone() else {return;};
            if 
                !available_ability_data.validate_selection(game, &incoming_selection) ||
                available_ability_data.grayed_out() ||
                *saved_selection == incoming_selection
            {
                return;
            }
        }
        

        let Some(player_saved_ability_inputs) = 
            game.saved_ability_inputs.players_saved_inputs.get_mut(&actor) else {return};

        let Some(SavedSingleAbility {
            selection: saved_selection,
            available_ability_data, 
        }) = 
            player_saved_ability_inputs.save.get_mut(&id) else {return;};

        if !available_ability_data.dont_save() {
            *saved_selection = incoming_selection.clone();
        }

        Self::send_selection_message(game, actor, id, incoming_selection);
        Self::send_saved_abilities(game, actor);
    }



    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        for (_, player_save) in game.saved_ability_inputs.players_saved_inputs.iter_mut(){
            for (_, saved_single_ability) in player_save.save.iter_mut(){
                saved_single_ability.reset_on_phase_start(phase);
            }
        }
        for player in PlayerReference::all_players(game){
            Self::send_saved_abilities(game, player);
        }
    }

    pub fn on_tick(game: &mut Game){
        for player in PlayerReference::all_players(game){
            let mut new_available_selection = 
                player.role_state(game).clone().available_ability_input(game, player);

            new_available_selection.combine_overwrite(
                SyndicateGunItem::available_ability_input(game, player)
            );
            new_available_selection.combine_overwrite(
                ForfeitVote::available_ability_input(game, player)
            );
            new_available_selection.combine_overwrite(
                Pitchfork::available_ability_input(game, player)
            );
            
            let current = Self::get_available_abilities_data(game, player);

            if current != new_available_selection {
                Self::set_available_ability_data(game, player, new_available_selection);
            }
        }
    }


    //mutators
    /// Keeps old selection if its valid, otherwise uses default_selection,
    /// even if default selection is invalid
    fn set_available_ability_data(
        game: &mut Game,
        player: PlayerReference,
        new_available_ability_data: AvailableAbilitiesData
    ){
        //set new available, try to keep old selection if possible
        
        game.saved_ability_inputs.players_saved_inputs.insert(player, 
            if let Some(current_saved_input) = game.saved_ability_inputs.players_saved_inputs.get(&player){

                let mut new_saved_input_map: VecMap<AbilityID, SavedSingleAbility> = VecMap::new();

                for (ability_id, new_single_ability_data) in new_available_ability_data.abilities().clone().into_iter() {
                    new_saved_input_map.insert(ability_id.clone(), SavedSingleAbility::new(
                        if let Some(SavedSingleAbility{selection: old_selection, ..}) = current_saved_input.save.get(&ability_id) {
                            if new_single_ability_data.validate_selection(game, old_selection){
                                old_selection.clone()
                            }else{
                                new_single_ability_data.default_selection().clone()
                            }
                        }else{
                            new_single_ability_data.default_selection().clone()
                        },
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
                            (id.clone(), SavedSingleAbility::new(selection, available_data))

                        })
                        .collect()
                )
            }
        );



        //inform client
        Self::send_saved_abilities(game, player);
    }

    fn send_saved_abilities(game: &Game, player: PlayerReference){
        if let Some(player_saved_input) = game.saved_ability_inputs.players_saved_inputs.get(&player) {
            player.send_packet(game, ToClientPacket::YourSavedAbilities { 
                save: player_saved_input.clone()
            });
        }
    }

    pub fn send_selection_message(
        game: &mut Game,
        player_ref: PlayerReference,
        id: AbilityID,
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
        game.saved_ability_inputs.players_saved_inputs
            .get(&player_ref).cloned()
            .unwrap_or_default()
    }

    pub fn get_available_abilities_data(game: &Game, player_ref: PlayerReference)->AvailableAbilitiesData{
        AvailableAbilitiesData::new(
            Self::get_player_saved_abilities(game, player_ref).save.into_iter()
                .map(|(id, saved_single_ability)| 
                    (id, saved_single_ability.available_ability_data))
                .collect()
        )
    }


    pub fn get_saved_ability_selection(
        game: &Game,
        player_ref: PlayerReference,
        id: AbilityID
    )->Option<AbilitySelection>{
        game.saved_ability_inputs.players_saved_inputs
            .get(&player_ref)
            .and_then(|data| data.save.get(&id))
            .map(|save_input| save_input.selection.clone())
    }

    pub fn get_two_role_outline_option_selection_if_id(
        game: &Game,
        player_ref: PlayerReference,
        id: AbilityID
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

//actual component
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSavedAbilities{
    save: VecMap<AbilityID, SavedSingleAbility>
}
impl PlayerSavedAbilities{
    fn new(save: VecMap<AbilityID, SavedSingleAbility>)->Self{
        Self{save}
    }
    // fn combine_mut(&mut self, other: Self){
        
    //     let mut new_map = VecMap::new();

    //     for (id, mut other_save) in other.save{
            
    //         other_save.selection = self.save.get(&id).map_or_else(
    //             || other_save.selection.clone(),
    //             |old_save| old_save.selection.clone()
    //         );

    //         new_map.insert(id, other_save);
    //     }

    //     self.save = new_map;
    // }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SavedSingleAbility{
    selection: AbilitySelection,

    available_ability_data: AvailableSingleAbilityData
}
impl SavedSingleAbility{
    fn new(selection: AbilitySelection, available_ability_data: AvailableSingleAbilityData)->Self{
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
