use serde::{Serialize, Deserialize};

use crate::game::player::PlayerIndex;
use crate::game::role::Role;

use super::ChatMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum NightInformation {
    RoleBlocked { immune : bool },  //you were roleblocked
    TargetSurvivedAttack,
    YouSurvivedAttack,
    YouDied,

    /* Role-specific */
    TargetJailed,

    SheriffResult { suspicious: bool },
    LookoutResult{players: Vec<PlayerIndex>},
    SeerResult{ enemies: bool},
    SpyMafiaVisit{players: Vec<PlayerIndex>},
    SpyCovenVisit{players: Vec<PlayerIndex>},
    SpyBug{message: Box<ChatMessage>},

    VeteranAttackedYou,
    VeteranAttackedVisitor,

    VigilanteSuicide,

    DoctorHealed, //Your target was attacked
    DoctorHealedYou, //"Someone attacked you but a doctor nursed you back to health"

    BodyguardProtected, //You redirected an attack off your target
    BodyguardProtectedYou, //You were attacked but a bodyguard protected you

    Transported,

    RetributionistBug{message: Box<ChatMessage>},
    NecromancerBug{message: Box<ChatMessage>},

    GodfatherForcedMafioso,
    GodfatherForcedYou,

    Silenced,

    PlayerRoleAndWill { role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    ConsigliereResult{ role: Role, visited_by: Vec<PlayerIndex>, visited: Vec<PlayerIndex>},
 
    WitchTargetImmune,
    WitchedYou { immune: bool },    //you were witched
    WitchBug{message: Box<ChatMessage>},

    ArsonistCleanedSelf,    //You cleaned the gas off yourself
    ArsonistWasDoused,  //you were doused in gas (only arsonists recieve this message)
}

impl NightInformation{
    pub fn to_chat_message(&self) -> ChatMessage {
        ChatMessage::NightInformation{night_information: self.clone()}
    }
    pub fn to_chat_message_vec(vec: &[Self]) -> Vec<ChatMessage> {
        vec.iter().map(|x| x.to_chat_message()).collect()
    }
}

