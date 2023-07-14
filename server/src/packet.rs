//! 
//! Examples of how serde serializes enum variants:
//! ```
//! use serde::{Serialize, Deserialize};
//! 
//! #[derive(Serialize, Deserialize)]
//! enum Test{
//!     Unit(i8),           // {"Unit": 6}
//!     Tuple(i8, bool),    // {"Tuple": [6, true]}
//!     Zero,               // "Zero"
//!     Struct{field: bool} // {"Struct": {"field": false}}
//! }
//! ```
//! Options:
//! Some(4).to_json_string()    // 4
//! None.to_json_string()       // null
//! 

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{game::{
    player::{PlayerIndex, PlayerReference},
    role_list::{RoleList, RoleListEntry},
    verdict::Verdict, phase::PhaseType, 
    chat::ChatMessage,
    role::{Role, RoleState, doomsayer::DoomsayerGuess}, 
    Game, grave::Grave, available_buttons::AvailableButtons, tag::Tag, settings::PhaseTimeSettings
}, listener::RoomCode, log};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ToClientPacket{
    
        //Pre lobby
    #[serde(rename_all = "camelCase")]
    AcceptJoin{in_game: bool},
    RejectJoin{reason: RejectJoinReason},
    #[serde(rename_all = "camelCase")]
    AcceptHost{room_code: RoomCode},
    
        //Lobby
    //Syncronize
    YourName{name: String},
    #[serde(rename_all = "camelCase")]
    YourPlayerIndex{player_index: PlayerIndex},
    Players{names: Vec<String>},
    Kicked,
    RejectStart{reason: RejectStartReason},
    YouAreHost,
    // 
    StartGame,

    #[serde(rename_all = "camelCase")]
    RoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    RoleListEntry{index: u8, role_list_entry: RoleListEntry},
    #[serde(rename_all = "camelCase")]
    PhaseTime{phase: PhaseType, time: u64},
    #[serde(rename_all = "camelCase")]
    PhaseTimes{phase_time_settings: PhaseTimeSettings},
    #[serde(rename_all = "camelCase")]
    ExcludedRoles{roles: Vec<RoleListEntry>},

        //Game
    //Syncronize
    #[serde(rename_all = "camelCase")]
    Phase{phase: PhaseType, day_number: u8, seconds_left: u64},   //Time left & PhaseType
    #[serde(rename_all = "camelCase")]
    PlayerOnTrial{player_index: PlayerIndex},  //Player index

        
    
    PlayerAlive{alive: Vec<bool>},
    #[serde(rename_all = "camelCase")]
    PlayerVotes{voted_for_player: HashMap<PlayerIndex, u8>}, //map from playerindex to num_voted_for that player

    YouAreSilenced,
    YouAreJailed,
    YourButtons{buttons: Vec<AvailableButtons>},
    #[serde(rename_all = "camelCase")]
    YourRoleLabels{role_labels: HashMap<PlayerIndex, Role>},
    #[serde(rename_all = "camelCase")]
    YourPlayerTags{player_tags: HashMap<PlayerIndex, Vec<Tag>>},
    YourWill{will: String},
    YourNotes{notes: String},
    #[serde(rename_all = "camelCase")]
    YourDeathNote{death_note: Option<String>},
    YourRole{role: Role},
    #[serde(rename_all = "camelCase")]
    YourRoleState{role_state: RoleState},
    #[serde(rename_all = "camelCase")]
    YourTarget{player_indices: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    YourVoting{player_index: Option<PlayerIndex>},
    YourJudgement{verdict: Verdict},
    //YourChatGroups{chat_groups: Vec<ChatGroup>},

    //Run function
    #[serde(rename_all = "camelCase")]
    AddChatMessages{chat_messages: Vec<ChatMessage>},
    AddGrave{grave: Grave},

    GameOver{reason: GameOverReason},

    //a way to syncronise the entire game for someone who joined late\
}
impl ToClientPacket {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self).map_err(|err|{
            log!(error "Serde error"; "Parsing JSON string: {:?}", self);
            err
        })
    }
    pub fn new_player_votes(game: &mut Game)->ToClientPacket{
        let mut voted_for_player: HashMap<PlayerIndex, u8> = HashMap::new();


        for player_ref in PlayerReference::all_players(game){
            if player_ref.alive(game){
                if let Some(player_voted) = player_ref.chosen_vote(game){
                    if let Some(num_votes) = voted_for_player.get_mut(&player_voted.index()){
                        *num_votes+=1;
                    }else{
                        voted_for_player.insert(player_voted.index(), 1);
                    }
                }
            }
        }

        ToClientPacket::PlayerVotes { voted_for_player }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectJoinReason {
    GameAlreadyStarted,
    RoomFull,
    InvalidRoomCode,
    ServerBusy,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectStartReason {
    GameEndsInstantly,
    ZeroTimeGame,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameOverReason {
    ReachedMaxDay,
    /*TODO Winner { who won? }*/
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToServerPacket{
    //Pre Lobby
    #[serde(rename_all = "camelCase")]
    Join{room_code: RoomCode},
    Host,

    //Lobby
    SetName{name: String},
    StartGame,
    #[serde(rename_all = "camelCase")]
    Kick{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    SetRoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    SetRoleListEntry{index: u8, role_list_entry: RoleListEntry},
    SetPhaseTime{phase: PhaseType, time: u64},
    #[serde(rename_all = "camelCase")]
    SetPhaseTimes{phase_time_settings: PhaseTimeSettings},
    SetExcludedRoles{roles: Vec<RoleListEntry>},

    //Game
    #[serde(rename_all = "camelCase")]
    Vote{player_index: Option<PlayerIndex>},   //Accusation
    Judgement{verdict: Verdict},  //Vote
    #[serde(rename_all = "camelCase")]
    Target{player_index_list: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    DayTarget{player_index:  PlayerIndex},

    SendMessage{text: String},
    #[serde(rename_all = "camelCase")]
    SendWhisper{player_index: PlayerIndex, text: String},
    SaveWill{will: String},
    SaveNotes{notes: String},
    #[serde(rename_all = "camelCase")]
    SaveDeathNote{death_note: Option<String>},

    Leave,

    //ROLE SPECIFIC PACKETS
    #[serde(rename_all = "camelCase")]
    SetForgerWill{ role: Role, will: String },
    
    #[serde(rename_all = "camelCase")]
    SetDoomsayerGuess{ guesses: [(PlayerReference, DoomsayerGuess); 3] },
}