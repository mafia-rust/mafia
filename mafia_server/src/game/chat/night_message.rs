use serde::{Serialize, Deserialize};

use crate::game::player::PlayerIndex;
use crate::game::role::Role;

use super::ChatMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NightInformation {
    RoleBlocked { immune : bool },
    TargetSurvivedAttack,
    YouSurvivedAttack,
    YouDied,

    /* Role-specific */
    
    SheriffResult { suspicious: bool },

    LookoutResult{players: Vec<PlayerIndex>},

    InvestigatorResult{roles: Vec<Role>},

    SpyMafiaVisit{players: Vec<PlayerIndex>},
    SpyBug{message: Box<ChatMessage>},

    VeteranAttacked,

    VigilanteSuicide,

    DoctorHealed,   //"Someone attacked you but a doctor nursed you back to health"
    DoctorHealedYou,

    BodyguardProtected,
    BodyguardProtectedYou,

    Transported,

    GodfatherForcedMafioso,
    GodfatherForcedYou,

    Blackmailed,

    ConsigliereResult(Role),

    FramerFramed(Vec<PlayerIndex>),


    JanitorResult { role: Role, will: String },

    ForgerResult { role: Role, will: String },

    WitchTargetImmune,
    WitchedYou { immune: bool },
    WitchBug{message: Box<ChatMessage>},

    ArsonistCleanedSelf,
    ArsonistDousedPlayers{players: Vec<PlayerIndex>},
}
