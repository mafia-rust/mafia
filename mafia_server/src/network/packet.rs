use std::collections::HashMap;

use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerIndex, role_list::RoleList, settings::{InvestigatorResults, PhaseTimeSettings}, vote::Verdict, phase::PhaseType}, lobby::LobbyIndex};

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

    PhaseTimesSetting,
    RoleList,
    InvestigatorResults,

    //Game
    //All of these are just for syncronizing variables between the 2 so client can see what their doing
    Phase{phase: PhaseType, seconds_left: u64},   //Time left & PhaseType
    PlayerOnTrial{player_index: PlayerIndex},  //Player index

    NewChatMessage,

    YourTarget,
    YourVoting,
    YourJudgement,
    YourWhispering,
    YourRole{},
    YourWill,

    ChatGroups,

    PlayerButtons,

    //a way to syncronise the entire game for someone who joined late
    //#endregion
}
impl ToClientPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
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