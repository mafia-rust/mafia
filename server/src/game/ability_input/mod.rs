pub mod common_selection;

use common_selection::{
    one_player_option_selection::OnePlayerOptionSelection, role_option_selection::RoleOptionSelection, two_role_option_selection::TwoRoleOptionSelection, two_role_outline_option_selection::TwoRoleOutlineOptionSelection, BooleanSelection
};
use serde::{Deserialize, Serialize};
use super::{
    components::generic_ability::GenericAbilitySelection,
    event::on_ability_input_received::OnAbilityInputReceived,
    player::PlayerReference,
    role::kira::KiraAbilityInput,
    Game
};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum AbilityInput{
    GenericAbility{selection: GenericAbilitySelection},

    //role abilities

    Disguiser{selection: RoleOptionSelection},
    Auditor{selection: TwoRoleOutlineOptionSelection},
    Steward{selection: TwoRoleOptionSelection},
    OjoInvestigate{selection: TwoRoleOutlineOptionSelection},
    Kira{selection: KiraAbilityInput},

    //Non role abilities

    ForfeitVote{selection: BooleanSelection},
    
    PitchforkVote{selection: OnePlayerOptionSelection},

    HitOrderVote{selection: OnePlayerOptionSelection},
    HitOrderMafioso,
}
impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        OnAbilityInputReceived::new(actor_ref, self.clone()).invoke(game);
    }
}