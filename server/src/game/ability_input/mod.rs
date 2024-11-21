pub mod common_input;

use common_input::{BooleanInput, OnePlayerOptionInput, RoleOptionSelection, TwoRoleOptionInput, TwoRoleOutlineOptionInput};
use serde::{Deserialize, Serialize};
use super::{
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference, 
    role::kira::KiraAbilityInput,
    Game
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum AbilityInput{
    Disguiser{input: RoleOptionSelection},
    Auditor{input: TwoRoleOutlineOptionInput},
    Steward{input: TwoRoleOptionInput},
    OjoInvestigate{input: TwoRoleOutlineOptionInput},
    Kira{input: KiraAbilityInput},

    //Non role abilities

    ForfeitVote{input: BooleanInput},
    
    PitchforkVote{input: OnePlayerOptionInput},

    HitOrderVote{input: OnePlayerOptionInput},
    HitOrderMafioso,
}
impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        OnAbilityInputReceived::new(actor_ref, self.clone()).invoke(game);
    }
}