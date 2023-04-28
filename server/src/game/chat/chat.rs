use serde::{Serialize, Deserialize};

use crate::game::{grave::Grave, role::{Role, RoleData}, player::{PlayerIndex, Player}, verdict::Verdict, phase::PhaseType, Game, role_list::{FactionAlignment, Faction}};
use super::night_message::NightInformation;

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
        
    /* System */
    Debug{text: String}, // TODO: Remove. This is only for testing.

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
    Targeted {
        targeter: PlayerIndex,
        target: Option<PlayerIndex> 
    },

    #[serde(rename_all = "camelCase")]
    NightInformation{
        night_information: NightInformation
    },

    /* Role-specific */
    #[serde(rename_all = "camelCase")]
    MayorRevealed{player_index: PlayerIndex}, //Sammy revealed as mayor
    MayorCantWhisper,   //you cant whisper as or to a revealed mayor
    JailorDecideExecuteYou,     //Jailor has decided to execute you
    MediumSeanceYou,       //You are being seanced by the medium
    JesterWon, //The jester will get their revenge from the grave
    ExecutionerWon, //You got your target lynched

    #[serde(rename_all = "camelCase")]
    PlayerWithNecronomicon{player_index: PlayerIndex}, //Sammy has the necronomicon

    #[serde(rename_all = "camelCase")]
    RoleData{role_data: RoleData},  //Tell executioner their target, other things. TODO

}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ChatGroup {
    All,
    Dead,

    Mafia,
    Vampire,
    Coven,

    //Jail
    //Seance
    //Whisper
    //Pirate, 
}
impl ChatGroup{
    pub fn player_recieve_from_chat_group(&self, player_index: PlayerIndex, game: &Game)->bool{
        let player = game.get_unchecked_player(player_index);
        match *self {
            ChatGroup::All => true,
            ChatGroup::Dead => !player.alive,   //or medium

            ChatGroup::Mafia => player.get_role().get_faction_alignment().faction() == Faction::Mafia,
            ChatGroup::Vampire => false,    //vampire
            ChatGroup::Coven => player.get_role().get_faction_alignment().faction() == Faction::Mafia,
        }
    }
    pub fn all_players_in_group(&self, game: &Game)->Vec<PlayerIndex>{
        let mut out = Vec::new();
        for player in game.players.iter(){
            if self.player_recieve_from_chat_group(player.index, game){
                out.push(player.index);
            }
        }
        out
    }
}
