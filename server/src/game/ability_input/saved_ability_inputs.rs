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

use super::{ability_selection::{AbilitySelection, AvailableAbilitySelection}, AbilityID, AbilityInput, AvailableAbilityInput, ValidateAvailableSelection};



//actual component
#[derive(Default)]
struct SavedAbilityInput{
    save: VecMap<AbilityID, (Option<AbilitySelection>, AvailableAbilitySelection)>
}
impl SavedAbilityInput{
    fn new(save: VecMap<AbilityID, (Option<AbilitySelection>, AvailableAbilitySelection)>)->Self{
        Self{save}
    }
}


//all players components
#[derive(Default)]
pub struct SavedAbilityInputs{
    players_saved_inputs: VecMap<PlayerReference, SavedAbilityInput>
}



impl SavedAbilityInputs{
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

            let Some((saved_selection, available_selection)) = 
                saved_ability_input.save.get(&id).clone() else {return;};
            if 
                !available_selection.validate_selection(game, &incoming_selection) ||
                saved_selection.as_ref().is_some_and(|s| *s == incoming_selection)
            {
                return;
            }
        }
        

        let Some(saved_ability_input) = 
            game.saved_ability_inputs.players_saved_inputs.get_mut(&actor) else {return};

        let Some((saved_selection, _)) = 
            saved_ability_input.save.get_mut(&id) else {return;};

        *saved_selection = Some(incoming_selection.clone());

        let out_packet = ToClientPacket::YourSavedAbilityInput{selection:
            saved_ability_input.save
                .iter()
                .filter_map(|(id, (selection, _))|{
                    if let Some(selection) = selection{
                        Some((id.clone(), selection.clone()))
                    }else{
                        None
                    }
                })
                .collect()
        };

        actor.send_packet(game, out_packet);
        Self::send_selection_message(game, actor, id, incoming_selection);
    }



    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        if phase != PhaseType::Obituary {return}
        game.saved_ability_inputs.players_saved_inputs.clear();
        for player in PlayerReference::all_players(game){
            player.send_packet(game, ToClientPacket::YourSavedAbilityInput { selection: vec![].into_iter().collect() });
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
            
            let current = Self::current_available_ability_input(game, player);

            if
                current.is_none() ||
                current.is_some_and(|c| c != new_available_selection)
            {
                Self::set_available_ability_input(game, player, new_available_selection);
            }
        }
    }


    fn set_available_ability_input(
        game: &mut Game,
        player: PlayerReference,
        new_available: AvailableAbilityInput
    ){
        //set new available, try to keep old selection if possible
        
        game.saved_ability_inputs.players_saved_inputs.insert(player, SavedAbilityInput::new(
            if let Some(current_saved_input) = game.saved_ability_inputs.players_saved_inputs.get(&player){

                let mut new_saved_input_map: VecMap<AbilityID, (Option<AbilitySelection>, AvailableAbilitySelection)> = VecMap::new();

                for (ability_id, new_available) in new_available.abilities {
                    //set new available, throwing out old_selection
                    new_saved_input_map.insert(ability_id.clone(), (None, new_available.clone()));

                    //then, if old selection is valid, keep it
                    if let Some((old_selection, _)) = current_saved_input.save.get(&ability_id) {
                        if let Some(old_selection) = old_selection{ 
                            if new_available.validate_selection(game, old_selection){
                                new_saved_input_map.insert(ability_id, (Some(old_selection.clone()), new_available));
                            }
                        }
                    }
                }

                new_saved_input_map
            }else{
                new_available.abilities
                    .iter()
                    .map(|(id, available_selection)| (id.clone(), (None, available_selection.clone())))
                    .collect()
            }
        ));



        //inform client
        let Some(player_saved_input) = game.saved_ability_inputs.players_saved_inputs.get(&player) else {return};

        player.send_packet(game, ToClientPacket::YourSavedAbilityInput{selection:
            player_saved_input.save
                .iter()
                .filter_map(|(id, (selection, _))|{
                    if let Some(selection) = selection{
                        Some((id.clone(), selection.clone()))
                    }else{
                        None
                    }
                })
                .collect()
        });

        player.send_packet(game, ToClientPacket::YourAvailableAbilityInput{selection: 
            player_saved_input.save
                .iter()
                .map(|(id, (_, available))|{
                    (id.clone(), available.clone())
                })
                .collect()
        });
    }

    fn current_available_ability_input(
        game: &Game,
        player_ref: PlayerReference
    )->Option<AvailableAbilityInput>{
        game.saved_ability_inputs.players_saved_inputs
            .get(&player_ref)
            .map(|data|
                AvailableAbilityInput::new(
                    data.save
                        .iter()
                        .map(|(id, (_, available_selection))|
                            (id.clone(), available_selection.clone())
                        )
                        .collect()
                )
            )
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



    //Query
    pub fn get_saved_ability_selection(
        game: &Game,
        player_ref: PlayerReference,
        id: AbilityID
    )->Option<AbilitySelection>{
        game.saved_ability_inputs.players_saved_inputs
            .get(&player_ref)
            .and_then(|data| data.save.get(&id))
            .map(|(selection, _)| selection.clone())
            .flatten()
    }

}

