use serde::{Deserialize, Serialize};

use crate::game::{
    grave::Grave,
    phase::PhaseState,
    player::{PlayerIndex, PlayerReference},
    role::{
        auditor::AuditorResult, engineer::TrapState, kira::KiraResult, ojo::OjoAction, puppeteer::PuppeteerAction, spy::SpyBug, Role
    },
    role_list::RoleOutline,
    verdict::Verdict, Game
};

use super::{ChatMessage, Recipient};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessageSender {
    Player{player: PlayerIndex},
    Jailor,
    Journalist,
    LivingToDead{player: PlayerIndex},
}

// Determines message color
#[derive(PartialOrd, Ord, Clone, Debug, Serialize, PartialEq, Eq)]
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

    
    #[serde(rename_all = "camelCase")]
    GameOver,
    #[serde(rename_all = "camelCase")]
    PlayerWonOrLost{player: PlayerIndex, won: bool, role: Role},
    #[serde(rename_all = "camelCase")]
    PlayerQuit{player_index: PlayerIndex},


    
    #[serde(rename_all = "camelCase")]
    PhaseChange{
        phase: PhaseState, 
        day_number: u8
    },
    /* Trial */
    #[serde(rename_all = "camelCase")]
    TrialInformation{
        required_votes: usize, 
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
    Targeted {
        targeter: PlayerIndex,
        targets: Vec<PlayerIndex>
    },

    #[serde(rename_all = "camelCase")]
    PhaseFastForwarded,

    /* Role-specific */
    #[serde(rename_all = "camelCase")]
    MayorRevealed{player_index: PlayerIndex},
    MayorCantWhisper,
    #[serde(rename_all = "camelCase")]
    JournalistJournal{journal: String},
    #[serde(rename_all = "camelCase")]
    YouAreInterviewingPlayer{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    PlayerIsBeingInterviewed{player_index: PlayerIndex},

    #[serde(rename_all = "camelCase")]
    JailedTarget{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    JailedSomeone{player_index: PlayerIndex},
    JailorDecideExecute {target: Option<PlayerIndex>},
    MediumHauntStarted{medium: PlayerIndex, player: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    DeputyKilled{shot_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    DeputyShotYou,

    PuppeteerPlayerIsNowMarionette{player: PlayerIndex},
    PuppeteerYouArePoisoned,


    #[serde(rename_all = "camelCase")]
    PlayerWithNecronomicon{player_index: PlayerIndex},
    YourConvertFailed,
    ApostleCanConvertTonight,
    ApostleCantConvertTonight,
    CultSacrificesRequired{required: u8},

    SomeoneSurvivedYourAttack,
    YouSurvivedAttack,
    TargetWasAttacked,
    YouWereProtected,
    YouDied,
    YouWereAttacked,
    YouAttackedSomeone,

    /*
    Night Information
    */
    RoleBlocked { immune : bool },

    Wardblocked,

    SheriffResult {suspicious: bool},
    LookoutResult{players: Vec<PlayerIndex>},
    TrackerResult{players: Vec<PlayerIndex>},
    SeerResult{enemies: bool},
    SpyMafiaVisit{players: Vec<PlayerIndex>},
    SpyCultistCount{count: u8},
    SpyBug{bug: SpyBug},
    PsychicGood{players: [PlayerIndex; 2]},
    PsychicEvil{players: [PlayerIndex; 3]},
    PsychicFailed,
    #[serde(rename_all = "camelCase")]
    AuditorResult{role_outline: RoleOutline, result: AuditorResult},
    SnoopResult{townie: bool},
    GossipResult{enemies: bool},

    EngineerVisitorsRole{role: Role},
    TrapState{state: TrapState},

    Transported,

    Silenced,
    #[serde(rename_all = "camelCase")]
    GodfatherBackup{backup: Option<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    GodfatherBackupKilled{backup: PlayerIndex},

    #[serde(rename_all = "camelCase")]
    EngineerRemoveTrap{unset: bool},
    

    #[serde(rename_all = "camelCase")]
    PlayerRoleAndAlibi { player: PlayerReference, role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    InformantResult{ role: Role, visited_by: Vec<PlayerIndex>, visited: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    ScarecrowResult{players: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    OjoSelection{action: OjoAction},
    #[serde(rename_all = "camelCase")]
    PuppeteerActionChosen{action: PuppeteerAction},
    #[serde(rename_all = "camelCase")]
    MarksmanChosenMarks{marks: Vec<PlayerIndex>},

    TargetIsPossessionImmune,
    YouWerePossessed { immune: bool },
    TargetsMessage{message: Box<ChatMessageVariant>},
    PossessionTargetsRole { role: Role },

    #[serde(rename_all = "camelCase")]
    WerewolfTrackingResult{tracked_player: PlayerIndex, players: Vec<PlayerIndex>},

    JesterWon,
    ProvocateurWon,
    DeathCollectedSouls,
    DoomsayerWon,
    DoomsayerFailed,
    KiraResult{result: KiraResult},
    MartyrRevealed { martyr: PlayerIndex },
    MartyrWon,
    MartyrFailed,
    WildcardConvertFailed{ role: Role }
}

impl ChatMessageVariant {
    pub fn send_to(self, game: &mut Game, recipient: impl Into<Recipient>) {
        ChatMessage::new(self, recipient).send(game)
    }
}