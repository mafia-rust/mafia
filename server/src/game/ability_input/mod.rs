pub mod selection_type;
pub mod ability_selection;
pub mod saved_ability_inputs;

use ability_selection::{AbilitySelection, AvailableAbilitySelection};

use serde::{Deserialize, Serialize};
use crate::vec_map::VecMap;

use super::{
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference,
    Game
};



pub type RoleAbilityID = u8;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum AbilityID{
    Role{role_ability_id: RoleAbilityID},
    ForfeitVote,
    PitchforkVote,
    SyndicateGunItemShoot,
    SyndicateGunItemGive,
}






#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AbilityInput{
    abilities: VecMap<AbilityID, AbilitySelection>
}


#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAbilityInput{
    abilities: VecMap<AbilityID, AvailableAbilitySelection>
}
impl AvailableAbilityInput{
    pub fn new(abilities: VecMap<AbilityID, AvailableAbilitySelection>)->Self{
        Self{abilities}
    }
}





pub trait ValidateAvailableSelection{
    type Selection;
    fn validate_selection(&self, selection: &Self::Selection)->bool;
}







// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
// #[serde(rename_all = "camelCase", tag="type")]
// pub enum AbilityInput{
//     GenericAbility{selection: GenericAbilitySelection},

//     //role abilities

//     Disguiser{selection: RoleOptionSelection},
//     Auditor{selection: TwoRoleOutlineOptionSelection},
//     Steward{selection: TwoRoleOptionSelection},
//     OjoInvestigate{selection: TwoRoleOutlineOptionSelection},
//     Kira{selection: KiraAbilityInput},

//     //Non role abilities

//     ForfeitVote{selection: BooleanSelection},
    
//     PitchforkVote{selection: OnePlayerOptionSelection},

//     SyndicateGunItemShoot{input: OnePlayerOptionSelection},
//     SyndicateGunItemGive{input: OnePlayerOptionSelection},

// }

impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        OnAbilityInputReceived::new(actor_ref, self.clone()).invoke(game);
    }
}