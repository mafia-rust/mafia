use std::collections::HashMap;

use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerIndex, role_list::RoleList, settings::{InvestigatorResults, PhaseTimeSettings}}, lobby::LobbyIndex};

#[derive(Serialize, Debug)]
pub enum ToClientPacket{

    //#region h
    AcceptJoin,
    RejectJoin{reason: String},
    AcceptHost{room_code: String},
    //#endregion
    
    //#region Lobby
    OpenGameMenu,
    YourName{name:String},
    Players{names: HashMap<PlayerIndex, String>},
    Kicked,

    PhaseTimesSetting,
    RoleList,
    InvestigatorResults,
    //#endregion


    ////////All of these are just for syncronizing variables between the 2 so client can see what their doing
    Phase,   //how much time is left with this
    PlayerOnTrial,

    NewChatMessage,

    YourTarget,
    YourVoting,
    YourJudgement,
    YourWhispering,
    YourRole{
        
    },
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

    Join{
        lobby_index: LobbyIndex
    },
    Host,

    //
    SetName{name: String},
    StartGame,
    Kick(PlayerIndex),
    SetRoleList(RoleList),
    SetPhaseTimes{phase_times: PhaseTimeSettings},
    SetInvestigatorResults(InvestigatorResults),

    //
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    // Whisper,
    SendMessage,
    SaveWill,
}
impl ToServerPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}