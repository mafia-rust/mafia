use crate::game::player::PlayerIndex;
use crate::game::role::Role;

#[derive(Clone)]
pub enum NightInformationMessage {
    RoleBlocked { immune : bool },
    Attacked { survived: bool },
    TargetSurvivedAttack,

    /* Role-specific */
    
    SheriffResult { suspicious: bool },

    LookoutResult(Box<[PlayerIndex]>),

    InvestigatorResult(/*InvestigativeResult*/),

    SpyMafiaVisit(Box<[PlayerIndex]>),
    SpyBug(Box<NightInformationMessage>),

    VeteranAlertsLeft(usize),
    VeteranAttacked,

    VigilanteSuicide,

    DoctorHealed,
    DoctorHealedYou,

    BodyguardProtected,
    BodyguardProtectedYou,

    Transported,

    GodfatherForcedMafioso,
    GodfatherForcedYou,

    Blackmailed,

    ConsigliereResult(Role),

    FramerFramed(Box<[PlayerIndex]>),


    JanitorResult { role: Role, will: String },

    ForgerResult { role: Role, will: String },

    WitchTargetImmune,
    WitchedYou { immune: bool },
    WitchBug(Box<NightInformationMessage>),

    ArsonistCleanedSelf,
}
