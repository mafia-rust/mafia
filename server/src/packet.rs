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
    role_list::{RoleList, RoleOutline},
    verdict::Verdict, phase::PhaseType, 
    chat::ChatMessage,
    role::{Role, RoleState, doomsayer::DoomsayerGuess}, 
    Game, grave::Grave, available_buttons::AvailableButtons, tag::Tag, settings::PhaseTimeSettings, RejectStartReason, GameOverReason
}, listener::{RoomCode, PlayerID}, log};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ToClientPacket{
    
    #[serde(rename_all = "camelCase")]
    RateLimitExceeded,

    // Pre lobby
    #[serde(rename_all = "camelCase")]
    LobbyList{
        room_codes: Vec<RoomCode>,
    },
    #[serde(rename_all = "camelCase")]
    AcceptJoin{room_code: RoomCode, in_game: bool, player_id: PlayerID},
    RejectJoin{reason: RejectJoinReason},
    
    // Lobby
    #[serde(rename_all = "camelCase")]
    YourId{player_id: PlayerID},
    #[serde(rename_all = "camelCase")]
    LobbyPlayers{players: HashMap<PlayerID, String>},
    #[serde(rename_all = "camelCase")]
    RejectStart{reason: RejectStartReason},
    PlayersHost{hosts: Vec<PlayerID>},
    #[serde(rename_all = "camelCase")]
    PlayersLostConnection{lost_connection: Vec<PlayerID>},
    StartGame,

    GamePlayers{players: Vec<String>},
    #[serde(rename_all = "camelCase")]
    RoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    RoleOutline{index: u8, role_outline: RoleOutline},
    #[serde(rename_all = "camelCase")]
    PhaseTime{phase: PhaseType, time: u64},
    #[serde(rename_all = "camelCase")]
    PhaseTimes{phase_time_settings: PhaseTimeSettings},
    #[serde(rename_all = "camelCase")]
    ExcludedRoles{roles: Vec<RoleOutline>},

    // Game
    
    #[serde(rename_all = "camelCase")]
    YourPlayerIndex{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    Phase{phase: PhaseType, day_number: u8, seconds_left: u64},
    #[serde(rename_all = "camelCase")]
    PlayerOnTrial{player_index: PlayerIndex},

    PlayerAlive{alive: Vec<bool>},
    #[serde(rename_all = "camelCase")]
    PlayerVotes{votes_for_player: HashMap<PlayerIndex, u8>},

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
    #[serde(rename_all = "camelCase")]
    YourRoleState{role_state: RoleState},
    #[serde(rename_all = "camelCase")]
    YourTarget{player_indices: Vec<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    YourVoting{player_index: Option<PlayerIndex>},
    YourJudgement{verdict: Verdict},

    #[serde(rename_all = "camelCase")]
    AddChatMessages{chat_messages: Vec<ChatMessage>},
    AddGrave{grave: Grave},

    GameOver{reason: GameOverReason},
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

        ToClientPacket::PlayerVotes { votes_for_player: voted_for_player }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RejectJoinReason {
    GameAlreadyStarted,
    RoomFull,
    RoomDoesntExist,
    ServerBusy,

    PlayerTaken,
    PlayerDoesntExist,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToServerPacket{
    // Pre Lobby
    LobbyListRequest,
    #[serde(rename_all = "camelCase")]
    ReJoin{room_code: RoomCode, player_id: PlayerID},
    #[serde(rename_all = "camelCase")]
    Join{room_code: RoomCode},
    Host,
    Leave,

    // Lobby
    SetName{name: String},
    StartGame,
    #[serde(rename_all = "camelCase")]
    SetRoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    SetRoleOutline{index: u8, role_outline: RoleOutline},
    SetPhaseTime{phase: PhaseType, time: u64},
    #[serde(rename_all = "camelCase")]
    SetPhaseTimes{phase_time_settings: PhaseTimeSettings},
    SetExcludedRoles{roles: Vec<RoleOutline>},

    // Game
    #[serde(rename_all = "camelCase")]
    Vote{player_index: Option<PlayerIndex>},
    Judgement{verdict: Verdict},
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

    // Role-specific
    #[serde(rename_all = "camelCase")]
    SetForgerWill{ role: Role, will: String },
    #[serde(rename_all = "camelCase")]
    SetDoomsayerGuess{ guesses: [(PlayerReference, DoomsayerGuess); 3] },
    #[serde(rename_all = "camelCase")]
    SetAmnesiacRoleOutline{ role_outline: RoleOutline },
}