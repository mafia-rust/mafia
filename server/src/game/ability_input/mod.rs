pub mod common_selection;

use common_selection::{
    one_player_option_selection::OnePlayerOptionSelection,
    role_option_selection::RoleOptionSelection,
    two_player_option_selection::TwoPlayerOptionSelection,
    two_role_option_selection::TwoRoleOptionSelection,
    two_role_outline_option_selection::TwoRoleOutlineOptionSelection,
    BooleanSelection
};
use serde::{Deserialize, Serialize};
use super::{
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference,
    Game
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct AbilityInput(AbilityID, AbilitySelection);


pub type RoleAbilityID = u8;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AbilityID{
    Role{
        role_ability_id: RoleAbilityID,
    },
    ForfeitVote,
    PitchforkVote,
    SyndicateGunItemShoot,
    SyndicateGunItemGive,
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AbilitySelection{
    Unit,
    BooleanSelection{selection: BooleanSelection},
    OnePlayerOption{selection: OnePlayerOptionSelection},
    TwoPlayerOption{selection: TwoPlayerOptionSelection},
    RoleOption{selection: RoleOptionSelection,},
    TwoRoleOption{selection: TwoRoleOptionSelection},
    TwoRoleOutlineOption{selection: TwoRoleOutlineOptionSelection},
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

//     HitOrderVote{selection: OnePlayerOptionSelection},

//     SyndicateGunItemShoot{input: OnePlayerOptionSelection},
//     SyndicateGunItemGive{input: OnePlayerOptionSelection},

//     HitOrderMafioso,
// }
impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        OnAbilityInputReceived::new(actor_ref, self.clone()).invoke(game);
    }
}