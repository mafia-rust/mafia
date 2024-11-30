
use serde::{Deserialize, Serialize};

use crate::{game::{phase::PhaseType, player::PlayerReference, Game}, vec_map::{vec_map, VecMap}, vec_set::VecSet};

use super::{
    ability_id::AbilityID,
    ability_selection::{AbilitySelection, AvailableAbilitySelection},
    selection_type::one_player_option_selection::{AvailableOnePlayerOptionSelection, OnePlayerOptionSelection},
};

pub mod available_single_ability_data;
use available_single_ability_data::*;

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAbilitiesData{
    abilities: VecMap<AbilityID, AvailableSingleAbilityData>
}
impl AvailableAbilitiesData{
    pub fn new(abilities: VecMap<AbilityID, AvailableSingleAbilityData>)->Self{
        Self{abilities}
    }
    pub fn new_ability(id: AbilityID, ability_data: AvailableSingleAbilityData)->Self{
        Self{
            abilities: vec_map!((id, ability_data))
        }
    }
    pub fn new_ability_fast(
        game: &Game,
        id: AbilityID,
        available: AvailableAbilitySelection,
        default_selection: AbilitySelection,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>
    )->Self{
        Self{abilities: vec_map![(id, AvailableSingleAbilityData::new(
                game,
                available,
                grayed_out,
                reset_on_phase_start,
                default_selection
        ))]}
    }
    pub fn new_one_player_ability_fast(
        game: &Game,
        id: AbilityID,
        available_players: VecSet<PlayerReference>,
        default_selection: Option<PlayerReference>,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>
    )->Self{


        Self{abilities: vec_map![(id, AvailableSingleAbilityData::new(
                game,
                AvailableAbilitySelection::OnePlayerOption { selection: AvailableOnePlayerOptionSelection(
                    available_players.into_iter().map(|p|Some(p)).collect()
                ) },
                grayed_out,
                reset_on_phase_start,
                AbilitySelection::OnePlayerOption { selection: OnePlayerOptionSelection(default_selection) }
        ))]}
    }
    pub fn insert_ability(&mut self, id: AbilityID, ability_data: AvailableSingleAbilityData){
        self.abilities.insert(id, ability_data);
    }
    pub fn combine_overwrite(&mut self, other: Self){
        for (ability_id, ability_selection) in other.abilities.into_iter(){
            self.abilities.insert(ability_id, ability_selection);
        }
    }
    pub fn abilities(&self)->&VecMap<AbilityID, AvailableSingleAbilityData>{
        &self.abilities
    }
}



