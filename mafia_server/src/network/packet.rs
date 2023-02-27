use std::collections::HashMap;

use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerIndex, role_list::RoleList, settings::{InvestigatorResults, PhaseTimeSettings}, vote::Verdict, phase::PhaseType, chat::{ChatMessage, ChatGroup}, role::Role}, lobby::LobbyIndex};

#[derive(Serialize, Debug)]
pub enum ToClientPacket{

        //Pre lobby
    AcceptJoin,
    RejectJoin{reason: String},
    AcceptHost{room_code: String},
    
        //Lobby
    OpenGameMenu,
    YourName{name:String},
    Players{names: HashMap<PlayerIndex, String>},
    Kicked,

    RoleList{role_list: RoleList},
    PhaseTimes{phase_times: PhaseTimeSettings},
    InvestigatorResults{investigator_results: InvestigatorResults},

        //Game
    //Syncronize
    Phase{phase: PhaseType, seconds_left: u64},   //Time left & PhaseType
    PlayerOnTrial{player_index: PlayerIndex},  //Player index
    YourWill{will: String},
    YourRole{role: Role},
    
    PlayerButtons{buttons: Vec<PlayerButtons>},

    //YourChatGroups{chat_groups: Vec<ChatGroup>},

    //Run function
    AddChatMessages{chat_messages: Vec<ChatMessage>},

    YourTarget{player_indices: Vec<PlayerIndex>},
    YourVoting,
    YourJudgement,

    //a way to syncronise the entire game for someone who joined late
}
impl ToClientPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerButtons{
    pub vote: bool,
    pub target: bool,
    pub day_target: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ToServerPacket{

    //Pre Lobby
    Join{
        lobby_index: LobbyIndex
    },
    Host,

    //Lobby
    SetName{name: String},
    StartGame,
    Kick{player_index: PlayerIndex},
    SetRoleList{role_list: RoleList},
    SetPhaseTimes{phase_times: PhaseTimeSettings},
    SetInvestigatorResults{investigator_results: InvestigatorResults},

    //Game
    Vote{player_index: PlayerIndex},   //Accusation
    Judgement{verdict: Verdict},  //Vote
    Target{player_index: Vec<PlayerIndex>},
    DayTarget{player_index:  PlayerIndex},

    SendMessage{text: String},
    SendWhisper{player_index: PlayerIndex, text: String},
    SaveWill{will: String},
}
impl ToServerPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}