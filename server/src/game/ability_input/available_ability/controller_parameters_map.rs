use std::iter::once;

use serde::{Deserialize, Serialize};

use crate::{
    game::{phase::PhaseType, player::PlayerReference, Game},
    vec_map::{vec_map, VecMap},
    vec_set::{vec_set, VecSet}
};

use super::super::{
    controller_id::ControllerID,
    ability_selection::AbilitySelection, AvailableAbilitySelection,
};

use super::controller_parameters::*;



#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerParametersMap{
    controllers: VecMap<ControllerID, ControllerParameters>
}
impl ControllerParametersMap{
    pub fn new(abilities: VecMap<ControllerID, ControllerParameters>)->Self{
        Self{controllers: abilities}
    }
    pub fn new_controller(id: ControllerID, ability_data: ControllerParameters)->Self{
        Self{
            controllers: vec_map!((id, ability_data))
        }
    }
    pub fn new_controller_fast(
        game: &Game,
        id: ControllerID,
        available: AvailableAbilitySelection,
        default_selection: AbilitySelection,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool,
        allowed_players: VecSet<PlayerReference>
    )->Self{
        if let Some(single) = ControllerParameters::new(
            game,
            available,
            grayed_out,
            reset_on_phase_start,
            dont_save,
            default_selection,
            allowed_players
        ){
            Self{controllers: vec_map![(id, single)]}
        }else{
            Self::default()
        }
    }
    pub fn new_one_player_ability_fast(
        game: &Game,
        actor: PlayerReference,
        id: ControllerID,
        available_players: VecSet<PlayerReference>,
        default_selection: Option<PlayerReference>,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool,
    )->Self{
        Self::new_controller_fast(
            game,
            id,
            AvailableAbilitySelection::new_one_player_option(
                available_players.into_iter().map(|p| Some(p)).chain(once(None)).collect()
            ),
            AbilitySelection::new_one_player_option(default_selection),
            grayed_out,
            reset_on_phase_start,
            dont_save,
            vec_set![actor]
        )
    }
    pub fn insert_ability(&mut self, id: ControllerID, ability_data: ControllerParameters){
        self.controllers.insert(id, ability_data);
    }
    pub fn combine_overwrite(&mut self, other: Self){
        for (ability_id, ability_selection) in other.controllers.into_iter(){
            self.controllers.insert(ability_id, ability_selection);
        }
    }
    pub fn combine_overwrite_owned(self, other: Self)->Self{
        let mut out = self;
        out.combine_overwrite(other);
        out
    }
    pub fn controller_parameters(&self)->&VecMap<ControllerID, ControllerParameters>{
        &self.controllers
    }
}