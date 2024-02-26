use serde::{Serialize, Deserialize};
use crate::game::{grave::Grave, role::Role, player::{PlayerIndex, PlayerReference}, verdict::Verdict, phase::PhaseType, Game};

use super::role::{spy::SpyBug, trapper::TrapState};

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
#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChatMessage {
    #[serde(rename_all = "camelCase")]
    Normal{
        message_sender: MessageSender, 
        text: String,
        chat_group: ChatGroup,
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

    
    #[serde(rename_all = "camelCase")]
    GameOver,
    #[serde(rename_all = "camelCase")]
    PlayerWonOrLost{player: PlayerIndex, won: bool, role: Role},
    #[serde(rename_all = "camelCase")]
    PlayerQuit{player_index: PlayerIndex},


    
    #[serde(rename_all = "camelCase")]
    PhaseChange{
        #[serde(rename = "phase")]
        phase_type: PhaseType, 
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

    VeteranAttackedYou,
    VeteranAttackedVisitor,

    TrapperVisitorsRole{role: Role},
    TrapperYouAttackedVisitor,
    TrapState{state: TrapState},

    Transported,

    Silenced,
    #[serde(rename_all = "camelCase")]
    GodfatherBackup{backup: Option<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    GodfatherBackupKilled{backup: PlayerIndex},
    

    #[serde(rename_all = "camelCase")]
    PlayerRoleAndWill { role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    ConsigliereResult{ role: Role, visited_by: Vec<PlayerIndex>, visited: Vec<PlayerIndex>},

    TargetIsPossessionImmune,
    YouWerePossessed { immune: bool },
    TargetsMessage{message: Box<ChatMessage>},

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
