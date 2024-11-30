use serde::{Deserialize, Serialize};

use crate::game::{ability_input::{ability_selection::{AbilitySelection, AvailableAbilitySelection}, ValidateAvailableSelection}, phase::PhaseType, Game};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableSingleAbilityData{
    available: AvailableAbilitySelection,
    grayed_out: bool,
    reset_on_phase_start: Option<PhaseType>,
    default_selection: AbilitySelection
}
impl AvailableSingleAbilityData{
    pub fn new(
        game: &Game,
        available: AvailableAbilitySelection,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        default_selection: AbilitySelection
    )->Option<Self>{
        if available.validate_selection(game, &default_selection) {
            Some(
                Self{
                    available,
                    grayed_out,
                    reset_on_phase_start,
                    default_selection
                }
            )
        }else{
            None
        }
    }

    ///assumes default selection is validated by available selection
    pub fn new_obituary_resetting_default_and_available(
        game: &Game,
        default_selection: AbilitySelection,
        available: AvailableAbilitySelection
    )->Option<Self>{
        Self::new(
            game,
            available,
            false,
            Some(PhaseType::Obituary),
            default_selection
        )
    }
    pub fn validate_selection(&self, game: &Game, selection: &AbilitySelection)->bool{
        self.available.validate_selection(game, selection)
    }
    pub fn default_selection(&self)->&AbilitySelection{
        &self.default_selection
    }
    pub fn set_grayed_out(&mut self, grayed_out: bool){
        self.grayed_out = grayed_out;
    }
    pub fn reset_on_phase_start(&self)->Option<PhaseType>{
        self.reset_on_phase_start
    }
}