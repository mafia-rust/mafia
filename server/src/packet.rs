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
use vec1::Vec1;


use crate::{
    client_connection::ClientConnection, game::{
        ability_input::*,
        chat::{ChatGroup, ChatMessage},
        components::insider_group::InsiderGroupID,
        grave::Grave, modifiers::ModifierType, phase::{PhaseState, PhaseType},
        player::{PlayerIndex, PlayerReference}, 
        role::{
            doomsayer::DoomsayerGuess,
            ClientRoleStateEnum, Role
        },
        role_list::{RoleList, RoleOutline}, settings::PhaseTimeSettings,
        tag::Tag, verdict::Verdict, GameOverReason, RejectStartReason
    }, listener::RoomCode, lobby::{game_client::GameClientLocation, lobby_client::{LobbyClient, LobbyClientID}}, log, vec_map::VecMap, vec_set::VecSet
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPreviewData {
    pub name: String,
    pub in_game: bool,
    pub players: Vec<(LobbyClientID, String)>
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HostDataPacketGameClient {
    pub client_type: GameClientLocation,
    pub connection: ClientConnection,
    pub host: bool,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ToClientPacket{
    Pong,
    
    #[serde(rename_all = "camelCase")]
    RateLimitExceeded,
    
    ForcedDisconnect,
    ForcedOutsideLobby,

    // Pre lobby
    #[serde(rename_all = "camelCase")]
    LobbyList{lobbies: HashMap<RoomCode, LobbyPreviewData>},
    #[serde(rename_all = "camelCase")]
    AcceptJoin{room_code: RoomCode, in_game: bool, player_id: LobbyClientID, spectator: bool},
    RejectJoin{reason: RejectJoinReason},
    
    // Lobby
    LobbyName{name: String},
    #[serde(rename_all = "camelCase")]
    YourId{player_id: LobbyClientID},
    #[serde(rename_all = "camelCase")]
    LobbyClients{clients: VecMap<LobbyClientID, LobbyClient>},
    PlayersHost{hosts: Vec<LobbyClientID>},
    PlayersReady{ready: Vec<LobbyClientID>},
    #[serde(rename_all = "camelCase")]
    PlayersLostConnection{lost_connection: Vec<LobbyClientID>},
    StartGame,
    #[serde(rename_all = "camelCase")]
    RejectStart{reason: RejectStartReason},

    // Settings
    #[serde(rename_all = "camelCase")]
    RoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    RoleOutline{index: u8, role_outline: RoleOutline},
    #[serde(rename_all = "camelCase")]
    PhaseTime{phase: PhaseType, time: u16},
    #[serde(rename_all = "camelCase")]
    PhaseTimes{phase_time_settings: PhaseTimeSettings},
    #[serde(rename_all = "camelCase")]
    EnabledRoles{roles: Vec<Role>},
    #[serde(rename_all = "camelCase")]
    EnabledModifiers{modifiers: Vec<ModifierType>},

    // Host
    HostData { clients: VecMap<LobbyClientID, HostDataPacketGameClient> },

    // Game
    GamePlayers{players: Vec<String>},
    GameInitializationComplete,
    BackToLobby,
    #[serde(rename_all = "camelCase")]
    YourPlayerIndex{player_index: PlayerIndex},
    #[serde(rename_all = "camelCase")]
    YourFellowInsiders{fellow_insiders: VecSet<PlayerIndex>},
    #[serde(rename_all = "camelCase")]
    Phase{phase: PhaseState, day_number: u8},
    #[serde(rename_all = "camelCase")]
    PhaseTimeLeft{seconds_left: Option<u16>},

    PlayerAlive{alive: Vec<bool>},
    #[serde(rename_all = "camelCase")]
    PlayerVotes{votes_for_player: VecMap<PlayerReference, u8>},

    #[serde(rename_all = "camelCase")]
    YourSendChatGroups{send_chat_groups: Vec<ChatGroup>},
    #[serde(rename_all = "camelCase")]
    YourInsiderGroups{insider_groups: VecSet<InsiderGroupID>},

    #[serde(rename_all = "camelCase")]
    YourAllowedControllers{
        save: VecMap<ControllerID, SavedController>
    },

    #[serde(rename_all = "camelCase")]
    YourRoleLabels{role_labels: VecMap<PlayerIndex, Role>},
    #[serde(rename_all = "camelCase")]
    YourPlayerTags{player_tags: VecMap<PlayerIndex, Vec1<Tag>>},
    YourWill{will: String},
    YourNotes{notes: Vec<String>},
    #[serde(rename_all = "camelCase")]
    YourCrossedOutOutlines{crossed_out_outlines: Vec<u8>},
    #[serde(rename_all = "camelCase")]
    YourDeathNote{death_note: Option<String>},
    #[serde(rename_all = "camelCase")]
    YourRoleState{role_state: ClientRoleStateEnum},
    #[serde(rename_all = "camelCase")]
    YourJudgement{verdict: Verdict},
    #[serde(rename_all = "camelCase")]
    YourVoteFastForwardPhase{fast_forward: bool},

    #[serde(rename_all = "camelCase")]
    AddChatMessages{chat_messages: Vec<ChatMessage>},
    AddGrave{grave: Grave},

    #[serde(rename_all = "camelCase")]
    NightMessages{chat_messages: Vec<ChatMessage>},

    GameOver{reason: GameOverReason},
}
impl ToClientPacket {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self).inspect_err(|_|{
            log!(error "Serde error"; "Parsing JSON string: {:?}", self);
        })
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
    Ping,
    // Pre Lobby
    LobbyListRequest,
    #[serde(rename_all = "camelCase")]
    ReJoin{room_code: RoomCode, player_id: LobbyClientID},
    #[serde(rename_all = "camelCase")]
    Join{room_code: RoomCode},
    Host,
    Leave,
    #[serde(rename_all = "camelCase")]
    Kick{player_id: LobbyClientID},
    #[serde(rename_all = "camelCase")]
    SetPlayerHost{player_id: LobbyClientID},
    RelinquishHost,

    // Lobby
    SendLobbyMessage{text: String},
    SetSpectator{spectator: bool},
    SetName{name: String},
    ReadyUp{ready: bool},
    SetLobbyName{name: String},
    StartGame,

    // Settings
    #[serde(rename_all = "camelCase")]
    SetRoleList{role_list: RoleList},
    #[serde(rename_all = "camelCase")]
    SetRoleOutline{index: u8, role_outline: RoleOutline},
    #[serde(rename_all = "camelCase")]
    SimplifyRoleList,
    #[serde(rename_all = "camelCase")]
    SetPhaseTime{phase: PhaseType, time: u16},
    #[serde(rename_all = "camelCase")]
    SetPhaseTimes{phase_time_settings: PhaseTimeSettings},
    #[serde(rename_all = "camelCase")]
    SetEnabledRoles{roles: Vec<Role>},
    #[serde(rename_all = "camelCase")]
    SetEnabledModifiers{modifiers: Vec<ModifierType>},

    // Host
    HostDataRequest,
    HostForceBackToLobby,
    HostForceEndGame,
    HostForceSkipPhase,
    HostForceSetPlayerName { id: LobbyClientID, name: String },

    // Game
    #[serde(rename_all = "camelCase")]
    Judgement{verdict: Verdict},

    SendChatMessage{text: String, block: bool},
    #[serde(rename_all = "camelCase")]
    SendWhisper{player_index: PlayerIndex, text: String},
    SaveWill{will: String},
    SaveNotes{notes: Vec<String>},
    #[serde(rename_all = "camelCase")]
    SaveCrossedOutOutlines{crossed_out_outlines: Vec<u8>},
    #[serde(rename_all = "camelCase")]
    SaveDeathNote{death_note: Option<String>},

    // AbilityInput
    #[serde(rename_all = "camelCase")]
    AbilityInput{ability_input: AbilityInput},
    // Role-specific
    #[serde(rename_all = "camelCase")]
    SetDoomsayerGuess{ guesses: [(PlayerReference, DoomsayerGuess); 3] },
    #[serde(rename_all = "camelCase")]
    SetConsortOptions{
        roleblock: bool,
        
        you_were_roleblocked_message: bool,
        you_survived_attack_message: bool,
        you_were_protected_message: bool,
        you_were_transported_message: bool,
        you_were_possessed_message: bool,
        your_target_was_jailed_message: bool,
    },
}