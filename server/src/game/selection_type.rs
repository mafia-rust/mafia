use serde::{Deserialize, Serialize};
use super::{player::PlayerReference, role::RoleState, role_outline_reference::RoleOutlineReference, Game};


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

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoRoleOutlineOptionInput(pub Option<RoleOutlineReference>, pub Option<RoleOutlineReference>);



#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase", tag="type")]
pub enum AbilityInput{
    Auditor{
        input: TwoRoleOutlineOptionInput
    },
    OjoInvestigate{
        input: TwoRoleOutlineOptionInput
    }
}
impl AbilityInput{
    pub fn on_client_message(self, game: &mut Game, actor_ref: PlayerReference){
        match self{
            Self::OjoInvestigate{input} => {
                if let RoleState::Ojo(mut ojo) = actor_ref.role_state(game).clone() {
                    
                    if let Some(outline) = input.0{
                        if !ojo.previously_given_results.iter().any(|(i, _)| *i == outline) {
                            ojo.chosen_outline.0 = Some(outline);
                        }
                    }else{
                        ojo.chosen_outline.0 = None;
                    }
                    if let Some(outline) = input.1{
                        if !ojo.previously_given_results.iter().any(|(i, _)| *i == outline) {
                            ojo.chosen_outline.1 = Some(outline);
                        }
                    }else{
                        ojo.chosen_outline.1 = None;
                    }
                    
                    if ojo.chosen_outline.0.is_some() && ojo.chosen_outline.1 == ojo.chosen_outline.0{
                        ojo.chosen_outline.1 = None;
                    }

                    actor_ref.set_role_state(game, ojo);
                }
            },
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
                    }else{
                        auditor.chosen_outline.1 = None;
                    }
                    
                    if auditor.chosen_outline.0.is_some() && auditor.chosen_outline.1 == auditor.chosen_outline.0{
                        auditor.chosen_outline.1 = None;
                    }

                    actor_ref.set_role_state(game, auditor);
                }
            }
        }
    }
}