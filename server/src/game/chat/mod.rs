use serde::{Serialize, Deserialize};
use crate::game::{grave::Grave, role::Role, player::{PlayerIndex, PlayerReference}, verdict::Verdict, Game};

use super::{phase::PhaseState, role::{auditor::AuditorResult, engineer::TrapState, ojo::OjoAction, spy::SpyBug}, role_list::RoleOutline};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessageSender {
    Player{player: PlayerIndex},
    Jailor,
    Journalist,
    LivingToDead{player: PlayerIndex},
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage{
    pub variant: ChatMessageVariant,
    pub chat_group: Option<ChatGroup>,
}
impl ChatMessage{
    pub fn new(variant: ChatMessageVariant, chat_group: Option<ChatGroup>)->Self{
        Self{variant,chat_group}
    }
    pub fn new_private(variant: ChatMessageVariant)->Self{
        Self{variant, chat_group: None}
    }
    pub fn new_non_private(variant: ChatMessageVariant, chat_group: ChatGroup)->Self{
        Self{variant, chat_group: Some(chat_group)}
    }
    pub fn get_variant(&self)->&ChatMessageVariant{
        &self.variant
    }
}


// Determines message color
#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChatMessageVariant {
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
    PlayerOnTrial{
        player_index: PlayerIndex
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

    /*
    Night Information
    */
    RoleBlocked { immune : bool },

    TargetJailed,

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

    VeteranAttackedYou,
    VeteranAttackedVisitor,

    CopAttackedVisitor,

    EngineerVisitorsRole{role: Role},
    EngineerYouAttackedVisitor,
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
    PlayerRoleAndWill { role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    ConsigliereResult{ role: Role, visited_by: Vec<PlayerIndex>, visited: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    OjoResult{players: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    OjoSelection{action: OjoAction},

    TargetIsPossessionImmune,
    YouWerePossessed { immune: bool },
    TargetsMessage{message: Box<ChatMessageVariant>},
    PossessionTargetsRole { role: Role },

    #[serde(rename_all = "camelCase")]
    WerewolfTrackingResult{tracked_player: PlayerIndex, players: Vec<PlayerIndex>},

    JesterWon,
    ExecutionerWon,
    DeathCollectedSouls,
    DoomsayerFailed,
    DoomsayerWon,
    MartyrRevealed { martyr: PlayerIndex },
    MartyrWon,
    MartyrFailed,
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum ChatGroup {
    All,
    Dead,

    Mafia,
    Cult,

    Jail,
    Interview
}
impl ChatGroup{
    pub fn player_receive_from_chat_group(&self, game: &Game, player_ref: PlayerReference)->bool{
        player_ref.get_current_receive_chat_groups(game).contains(self)
    }

    pub fn all_players_in_group(&self, game: &Game)->Vec<PlayerReference>{
        let mut out = Vec::new();
        for player_ref in PlayerReference::all_players(game){
            if self.player_receive_from_chat_group(game, player_ref){
                out.push(player_ref);
            }
        }
        out
    }
}
