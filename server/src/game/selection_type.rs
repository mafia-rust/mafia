use serde::{Deserialize, Serialize};
use super::{player::PlayerReference, role::{auditor::AuditorAbilityInput, RoleState}, Game};


// struct OnePlayerOptionSelection{
//     selection: Option<PlayerReference>
// }
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


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum AbilityInput{
    Auditor{
        input: AuditorAbilityInput
    }
}
impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        match self{
            Self::Auditor{input} => {
                if let RoleState::Auditor(mut auditor) = actor_ref.role_state(game).clone() {
                    
                    if let Some(outline) = input.0{
                        if !auditor.previously_given_results.iter().any(|(i, _)| *i == outline) {
                            auditor.chosen_outline.0 = Some(outline);
                        }
                    }else{
                        auditor.chosen_outline.0 = None;
                    }
                    if let Some(outline) = input.1{
                        if !auditor.previously_given_results.iter().any(|(i, _)| *i == outline) {
                            auditor.chosen_outline.1 = Some(outline);
                        }
                    }{
                        auditor.chosen_outline.1 = None;
                    }

                    actor_ref.set_role_state(game, auditor);
                }
            }
        }
    }
}