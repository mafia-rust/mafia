use crate::game::{grave::Grave, role::{Role, RoleData}, player::{PlayerIndex, Player}, vote::Verdict, phase::PhaseType};
use super::night_message::NightInformationMessage;

#[derive(Clone)]
pub enum MessageSender {
    Player(PlayerIndex),
    Jailor,
    Medium,
}

// Determines message color
#[derive(Clone)]
pub enum ChatMessage {
    Normal(MessageSender, String, ChatGroup),
    Whisper(PlayerIndex, String),
    /* System */
    Debug(String), // TODO: Remove. This is only for testing.

    RoleAssignment(Role),   //you are this role
    PlayerDied(Grave),      //this player died this is their role
    GameOver/*(WinState)*/,
    PhaseChange(PhaseType, u8),
    /* Trial */
    TrialInformation{required_votes: usize, trials_left: u8},

    Voted { voter: PlayerIndex, votee: PlayerIndex },
    VoteCleared { voter: PlayerIndex },

    PlayerOnTrial(PlayerIndex),     //This  player is on trial

    JudgementVote(PlayerIndex),             //Sammy voted
    JudgementVerdict(PlayerIndex, Verdict), //Sammy voted innocent
    JudgementResults {player_on_trial: PlayerIndex, innocent: usize, guilty: usize },    //Sammy was voted innocent with these many votes
    
    /* Misc */
    BroadcastWhisper { whisperer: PlayerIndex, whisperee: PlayerIndex },    //Sammy whispered to Tyler
    Targeted { targeter: PlayerIndex, target: PlayerIndex },                //Sammy targeted Jack
    TargetCleared { targeter: PlayerIndex },                                //Sammy cleared targets
    
    /* Role-specific */
    MayorRevealed(PlayerIndex), //Sammy revealed as mayor
    JailorDecideExecuteYou,     //Jailor has decided to execute you
    MediumSeanceYou,       //You are being seanced by the medium

    RoleData(RoleData)  //Tell executioner their target, other things. TODO
}

#[derive(Clone)]
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
