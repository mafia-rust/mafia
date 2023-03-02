use serde::{Serialize, Deserialize};

use crate::game::{grave::Grave, role::{Role, RoleData}, player::{PlayerIndex, Player}, vote::Verdict, phase::PhaseType};
use super::night_message::NightInformationMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageSender {
    Player(PlayerIndex),
    Jailor,
    Medium,
}

// Determines message color
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatMessage {
    Normal{message_sender: MessageSender, text: String, chat_group: ChatGroup},
    Whisper{from_player_index: PlayerIndex, to_player_index: PlayerIndex, text: String},
    /* System */
    Debug(String), // TODO: Remove. This is only for testing.

    RoleAssignment{role: Role},   //you are this role
    PlayerDied{grave: Grave},      //this player died this is their role
    GameOver/*(WinState)*/,
    PhaseChange{phase_type: PhaseType, day_number: u8},
    /* Trial */
    TrialInformation{required_votes: usize, trials_left: u8},

    Voted { voter: PlayerIndex, votee: Option<PlayerIndex> },

    PlayerOnTrial{player_index: PlayerIndex},     //This  player is on trial

    JudgementVote{voter_player_index: PlayerIndex},             //Sammy voted
    JudgementVerdict{voter_player_index: PlayerIndex, verdict: Verdict}, //Sammy voted innocent
    JudgementResults {player_on_trial: PlayerIndex, innocent: usize, guilty: usize },    //Sammy was voted innocent with these many votes
    
    /* Misc */
    BroadcastWhisper { whisperer: PlayerIndex, whisperee: PlayerIndex },    //Sammy whispered to Tyler
    Targeted { targeter: PlayerIndex, target: Option<PlayerIndex> },        //Sammy targeted Jack
    NightInformationMessage{ night_information: NightInformationMessage },

    /* Role-specific */
    MayorRevealed{player_index: PlayerIndex}, //Sammy revealed as mayor
    MayorCantWhisper,   //you cant whisper as or to a revealed mayor
    JailorDecideExecuteYou,     //Jailor has decided to execute you
    MediumSeanceYou,       //You are being seanced by the medium

    RoleData{role_data: RoleData}  //Tell executioner their target, other things. TODO
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatGroup {
    All,
    Mafia,
    Dead,
    Vampire,
    Coven,
    //Jail
    //Seance
    //Whisper
    //Pirate, 
}
