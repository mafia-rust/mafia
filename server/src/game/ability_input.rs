use serde::{Deserialize, Serialize};
use super::{
    event::on_ability_input_received::OnAbilityInputReceived, 
    player::PlayerReference, 
    role_outline_reference::RoleOutlineReference, 
    Game
};



// struct TwoPlayerOptionSelection{
//     selection: Option<(PlayerReference, PlayerReference)>
// }
// #[derive(Serialize, Deserialize)]
// struct RoleOutlineOptionSelection{
//     selection: Option<RoleOutlineReference>
// }
// #[derive(Serialize, Deserialize)]
// struct TwoRoleOutlineOptionSelection{
//     selection: (Option<RoleOutlineReference>, Option<RoleOutlineReference>)
// }

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BooleanInput(pub bool);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePlayerOptionInput(pub Option<PlayerReference>);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionInput(pub Option<RoleOutlineReference>, pub Option<RoleOutlineReference>);



#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum AbilityInput{
    Auditor{input: TwoRoleOutlineOptionInput},
    OjoInvestigate{input: TwoRoleOutlineOptionInput},
    

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