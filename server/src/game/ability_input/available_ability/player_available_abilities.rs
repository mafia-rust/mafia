use std::iter::once;

use serde::{Deserialize, Serialize};

use crate::{game::{phase::PhaseType, player::PlayerReference, Game}, vec_map::{vec_map, VecMap}, vec_set::VecSet};

use super::super::{
    ability_id::AbilityID,
    ability_selection::AbilitySelection, AvailableAbilitySelection,
};

use super::available_single_ability_data::*;



#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAvailableAbilities{
    abilities: VecMap<AbilityID, AvailableSingleAbilityData>
}
impl PlayerAvailableAbilities{
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
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool
    )->Self{
        if let Some(single) = AvailableSingleAbilityData::new(
            game,
            available,
            grayed_out,
            reset_on_phase_start,
            dont_save,
            default_selection
        ){
            Self{abilities: vec_map![(id, single)]}
        }else{
            Self::default()
        }
    }
    pub fn new_one_player_ability_fast(
        game: &Game,
        id: AbilityID,
        available_players: VecSet<PlayerReference>,
        default_selection: Option<PlayerReference>,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool
    )->Self{
        Self::new_ability_fast(
            game,
            id,
            AvailableAbilitySelection::new_one_player_option(
                available_players.into_iter().map(|p| Some(p)).chain(once(None)).collect()
            ),
            AbilitySelection::new_one_player_option(default_selection),
            grayed_out,
            reset_on_phase_start,
            dont_save
        )
    }
    pub fn insert_ability(&mut self, id: AbilityID, ability_data: AvailableSingleAbilityData){
        self.abilities.insert(id, ability_data);
    }
    pub fn combine_overwrite(&mut self, other: Self){
        for (ability_id, ability_selection) in other.abilities.into_iter(){
            self.abilities.insert(ability_id, ability_selection);
        }
    }
    pub fn combine_overwrite_owned(self, other: Self)->Self{
        let mut out = self;
        out.combine_overwrite(other);
        out
    }
    pub fn abilities(&self)->&VecMap<AbilityID, AvailableSingleAbilityData>{
        &self.abilities
    }
}