use serde::{Deserialize, Serialize};

use crate::game::{
    ability_input::*, components::synopsis::Synopsis, grave::Grave, phase::PhaseState, player::{PlayerIndex, PlayerReference}, role::{
        auditor::AuditorResult, engineer::TrapState, kira::KiraResult, krampus::KrampusAbility, santa_claus::SantaListKind, spy::SpyBug, Role
    }, role_list::RoleOutline, tag::Tag, verdict::Verdict, win_condition::WinCondition
};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessageSender {
    Player{player: PlayerIndex},
    Jailor,
    Reporter,
    LivingToDead{player: PlayerIndex},
}

// Determines message color
#[derive(PartialOrd, Ord, Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChatMessageVariant {
    LobbyMessage {
        sender: String,
        text: String
    },

    #[serde(rename_all = "camelCase")]
    Normal{
        message_sender: MessageSender, 
        text: String,
        block: bool,
    },

    #[serde(rename_all = "camelCase")]
    Whisper{
        from_player_index: PlayerIndex, 
        to_player_index: PlayerIndex, 
        text: String
    },

    BroadcastWhisper {
        whisperer: PlayerIndex, 
        whisperee: PlayerIndex 
    },

    RoleAssignment{role: Role},
    PlayerDied{grave: Grave},
    PlayersRoleRevealed{player: PlayerIndex, role: Role},
    PlayersRoleConcealed{player: PlayerIndex},
    TagAdded{player: PlayerIndex, tag: Tag},
    TagRemoved{player: PlayerIndex, tag: Tag},
    
    #[serde(rename_all = "camelCase")]
    GameOver { synopsis: Synopsis },
    #[serde(rename_all = "camelCase")]
    PlayerQuit{player_index: PlayerIndex, game_over: bool},


    
    #[serde(rename_all = "camelCase")]
    PhaseChange{
        phase: PhaseState, 
        day_number: u8
    },
    /* Trial */
    #[serde(rename_all = "camelCase")]
    TrialInformation{
        required_votes: u8, 
        trials_left: u8
    },

    #[serde(rename_all = "camelCase")]
    Voted {
        voter: PlayerIndex, 
        votee: Option<PlayerIndex> 
    },
    #[serde(rename_all = "camelCase")]
    PlayerNominated{
        player_index: PlayerIndex,
        players_voted: Vec<PlayerIndex>
    },
    #[serde(rename_all = "camelCase")]
    JudgementVerdict{
        voter_player_index: PlayerIndex, 
        verdict: Verdict
    },
    #[serde(rename_all = "camelCase")]
    TrialVerdict {
        player_on_trial: PlayerIndex, 
        innocent: u8, 
        guilty: u8 
    },
    
    /* Misc */
    #[serde(rename_all = "camelCase")]
    AbilityUsed{
        player: PlayerIndex,
        ability_id: ControllerID,
        selection: AbilitySelection
    },

    #[serde(rename_all = "camelCase")]
    PhaseFastForwarded,

    /* Role-specific */
    #[serde(rename_all = "camelCase")]
    MayorRevealed{player_index: PlayerIndex},
    InvalidWhisper,
    #[serde(rename_all = "camelCase")]
    PoliticianCountdownStarted,
    #[serde(rename_all = "camelCase")]
    ReporterReport{report: String},
    #[serde(rename_all = "camelCase")]
    PlayerIsBeingInterviewed{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    JailedTarget{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    JailedSomeone{player_index: PlayerIndex},
    MediumHauntStarted{medium: PlayerIndex, player: PlayerIndex},
    MediumExists,
    #[serde(rename_all = "camelCase")]
    DeputyKilled{shot_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    DeputyShotYou,
    #[serde(rename_all = "camelCase")]
    WardenPlayersImprisoned{players: Vec<PlayerReference>},
    WerewolfTracked,
    
    #[serde(rename_all = "camelCase")]
    PlayerDiedOfABrokenHeart{player: PlayerIndex, lover: PlayerIndex},

    PuppeteerPlayerIsNowMarionette{player: PlayerIndex},
    RecruiterPlayerIsNowRecruit{player: PlayerIndex},

    YourConvertFailed,
    CultConvertsNext,
    CultKillsNext,

    NextSantaAbility { ability: SantaListKind },
    AddedToNiceList,
    NextKrampusAbility { ability: KrampusAbility },
    AddedToNaughtyList,
    SantaAddedPlayerToNaughtyList { player: PlayerReference },

    SomeoneSurvivedYourAttack,
    YouSurvivedAttack,
    TargetWasAttacked,
    YouWereProtected,
    YouDied,
    YouWereAttacked,
    YouAttackedSomeone,

    YouArePoisoned,

    /*
    Night Information
    */
    RoleBlocked,
    Wardblocked,

    SheriffResult {suspicious: bool},
    LookoutResult{players: Vec<PlayerIndex>},
    TrackerResult{players: Vec<PlayerIndex>},
    SeerResult{enemies: bool},
    SpyMafiaVisit{players: Vec<PlayerIndex>},
    SpyBug{bug: SpyBug},
    PsychicGood{player: PlayerReference},
    PsychicEvil{first: PlayerReference, second: PlayerReference},
    PsychicFailed,
    #[serde(rename_all = "camelCase")]
    AuditorResult{role_outline: RoleOutline, result: AuditorResult},
    SnoopResult{townie: bool},
    GossipResult{enemies: bool},
    #[serde(rename_all = "camelCase")]
    TallyClerkResult{evil_count: u8},

    EngineerVisitorsRole{role: Role},
    TrapState{state: TrapState},
    TrapStateEndOfNight{state: TrapState},

    
    ArmorsmithArmorBroke,

    Transported,

    Silenced,
    #[serde(rename_all = "camelCase")]
    GodfatherBackup{backup: Option<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    GodfatherBackupKilled{backup: PlayerIndex},
    

    #[serde(rename_all = "camelCase")]
    PlayerRoleAndAlibi { player: PlayerReference, role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    InformantResult{ role: Role, visited_by: Vec<PlayerIndex>, visited: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    FramerResult{ mafia_member: PlayerIndex, visitors: Vec<Role>},
    #[serde(rename_all = "camelCase")]
    ScarecrowResult{players: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    AmbusherCaught{ambusher: PlayerReference},

    TargetIsPossessionImmune,
    YouWerePossessed { immune: bool },
    TargetsMessage{message: Box<ChatMessageVariant>},
    PlayerForwardedMessage{forwarder: PlayerReference, message: Box<ChatMessageVariant>},
    TargetHasRole { role: Role },
    #[serde(rename_all = "camelCase")]
    TargetHasWinCondition { win_condition: WinCondition },

    #[serde(rename_all = "camelCase")]
    WerewolfTrackingResult{tracked_player: PlayerIndex, players: Vec<PlayerIndex>},

    #[serde(rename_all = "camelCase")]
    YouAreLoveLinked{player: PlayerIndex},

    JesterWon,
    RevolutionaryWon,
    ChronokaiserSpeedUp{percent: u32},
    DoomsayerWon,
    DoomsayerFailed,
    KiraResult{result: KiraResult},
    MartyrRevealed { martyr: PlayerIndex },
    MartyrWon,
    MartyrFailed,
    WildcardConvertFailed{ role: Role },
    CowardHid,
    CowardSaved,
}