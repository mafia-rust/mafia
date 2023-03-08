use serde::{Serialize, Deserialize};

use crate::game::{grave::Grave, role::{Role, RoleData}, player::{PlayerIndex, Player}, verdict::Verdict, phase::PhaseType, Game, role_list::{FactionAlignment, Faction}};
use super::night_message::NightInformation;

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
    TrialVerdict {player_on_trial: PlayerIndex, innocent: usize, guilty: usize },    //Sammy was voted innocent with these many votes
    
    /* Misc */
    BroadcastWhisper { whisperer: PlayerIndex, whisperee: PlayerIndex },    //Sammy whispered to Tyler
    Targeted { targeter: PlayerIndex, target: Option<PlayerIndex> },        //Sammy targeted Jack
    NightInformation{ night_information: NightInformation },

    /* Role-specific */
    MayorRevealed{player_index: PlayerIndex}, //Sammy revealed as mayor
    MayorCantWhisper,   //you cant whisper as or to a revealed mayor
    JailorDecideExecuteYou,     //Jailor has decided to execute you
    MediumSeanceYou,       //You are being seanced by the medium

    RoleData{role_data: RoleData},  //Tell executioner their target, other things. TODO
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
impl ChatGroup{
    pub fn player_in_group(&self, game: &Game, playerIndex: PlayerIndex)->bool{
        let player = game.get_unchecked_player(playerIndex);
        match *self {
            ChatGroup::All => true,
            ChatGroup::Dead => !player.alive,   //or medium

            ChatGroup::Mafia => player.get_role().get_faction_alignment().faction() == Faction::Mafia,
            ChatGroup::Vampire => false,    //vampire
            ChatGroup::Coven => false,
        }
    }
    pub fn all_players_in_group(&self, game: &Game)->Vec<PlayerIndex>{
        let mut out = Vec::new();
        for player in game.players.iter(){
            if self.player_in_group(game, player.index){
                out.push(player.index);
            }
        }
        out
    }
}
