pub mod night_message;

use serde::{Serialize, Deserialize};

use crate::game::{grave::Grave, role::Role, player::{PlayerIndex, PlayerReference}, verdict::Verdict, phase::PhaseType, Game};
use self::night_message::NightInformation;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessageSender {
    Player {player: PlayerIndex},
    Jailor,
    Medium,
}

// Determines message color
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChatMessage {
    #[serde(rename_all = "camelCase")]
    Normal{
        message_sender: MessageSender, 
        text: String, 
        chat_group: ChatGroup
    },

    #[serde(rename_all = "camelCase")]
    Whisper{
        from_player_index: PlayerIndex, 
        to_player_index: PlayerIndex, 
        text: String
    },

    //Sammy whispered to Tyler
    BroadcastWhisper {
        whisperer: PlayerIndex, 
        whisperee: PlayerIndex 
    },

    RoleAssignment{role: Role},   //you are this role
    PlayerDied{grave: Grave},      //this player died this is their role
    GameOver/*(WinState)*/,


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

    //Geneveive is on trial
    #[serde(rename_all = "camelCase")]
    PlayerOnTrial{
        player_index: PlayerIndex
    },

    //Sammy voted
    #[serde(rename_all = "camelCase")]
    JudgementVote{
        voter_player_index: PlayerIndex
    },

    //Sammy voted innocent
    #[serde(rename_all = "camelCase")]
    JudgementVerdict{
        voter_player_index: PlayerIndex, 
        verdict: Verdict
    },

    //Sammy was voted innocent with these many votes
    #[serde(rename_all = "camelCase")]
    TrialVerdict {
        player_on_trial: PlayerIndex, 
        innocent: usize, 
        guilty: usize 
    },
    
    /* Misc */
    //Sammy targeted Jack
    #[serde(rename_all = "camelCase")]
    Targeted {
        targeter: PlayerIndex,
        targets: Vec<PlayerIndex> 
    },

    #[serde(rename_all = "camelCase")]
    NightInformation{
        night_information: NightInformation
    },

    /* Role-specific */
    #[serde(rename_all = "camelCase")]
    MayorRevealed{player_index: PlayerIndex}, //Sammy revealed as mayor
    MayorCantWhisper,   //you cant whisper as or to a revealed mayor
    #[serde(rename_all = "camelCase")]
    JailedTarget{player_index: PlayerIndex}, //You jailed Sammy
    #[serde(rename_all = "camelCase")]
    JailedSomeone{player_index: PlayerIndex}, //you have been jailed
    JailorDecideExecuteYou,     //Jailor has decided to execute you
    MediumSeanceYou,       //You are being seanced by the medium
    JesterWon, //The jester will get their revenge from the grave
    ExecutionerWon, //You got your target lynched

    #[serde(rename_all = "camelCase")]
    PlayerWithNecronomicon{player_index: PlayerIndex}, //Sammy has the necronomicon

    // #[serde(rename_all = "camelCase")]
    // RoleState{role_data: RoleState},  //Tell executioner their target, other things. TODO

}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ChatGroup {
    All,
    Dead,

    Mafia,
    Vampire,
    Coven,

    Jail,
    //Seance
    //Whisper
    //Pirate, 
}
impl ChatGroup{
    pub fn player_receive_from_chat_group(&self, game: &Game, player_ref: PlayerReference)->bool{
        player_ref.role_state_get_current_recieve_chat_groups(game).contains(self)
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
