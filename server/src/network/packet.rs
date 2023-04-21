use std::collections::HashMap;

use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::game::{
    player::{PlayerIndex, Player},
    role_list::RoleList,
    settings::{InvestigatorResults, PhaseTimeSettings},
    verdict::Verdict, phase::PhaseType, 
    chat::{ChatMessage, ChatGroup},
    role::{Role, RoleData}, 
    Game, grave::Grave
};

use super::listener::RoomCode;

#[derive(Serialize, Debug, Clone)]
pub enum ToClientPacket{
    
        //Pre lobby
    AcceptJoin,
    RejectJoin{reason: RejectJoinReason},
    AcceptHost{room_code: RoomCode},
    
        //Lobby
    //Syncronize
    YourName{name: String},
    YourPlayerIndex{player_index: PlayerIndex},
    Players{names: Vec<String>},
    Kicked,
    RejectStart{reason: RejectStartReason},
    // 
    StartGame,

    RoleList{role_list: RoleList},
    PhaseTime{phase: PhaseType, time: u64},
    InvestigatorResults{investigator_results: InvestigatorResults},

        //Game
    //Syncronize
    Phase{phase: PhaseType, day_number: u8, seconds_left: u64},   //Time left & PhaseType
    PlayerOnTrial{player_index: PlayerIndex},  //Player index

        
    PlayerButtons{buttons: Vec<PlayerButtons>},
    PlayerAlive{alive: Vec<bool>},
    PlayerVotes{voted_for_player: Vec<u8>}, //map from playerindex to num_voted_for that player

    YourWill{will: String},
    YourRole{role: RoleData},
    YourTarget{player_indices: Vec<PlayerIndex>},
    YourVoting{player_index: Option<PlayerIndex>},
    YourJudgement{verdict: Verdict},
    //YourChatGroups{chat_groups: Vec<ChatGroup>},

    //Run function
    AddChatMessages{chat_messages: Vec<ChatMessage>},
    AddGrave{grave: Grave},

    GameOver{reason: GameOverReason},

    //a way to syncronise the entire game for someone who joined late
}
impl ToClientPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
    pub fn new_PlayerVotes(game: &mut Game)->ToClientPacket{
        let mut voted_for_player: Vec<u8> = Vec::new();

        for _ in game.players.iter(){
            voted_for_player.push(0);
        }

        for player in game.players.iter(){
            if player.alive{
                if let Some(player_voted) = player.voting_variables.chosen_vote{
                    if let Some(num_votes) = voted_for_player.get_mut(player_voted as usize){
                        *num_votes+=1;
                    }
                }
            }
        }

        ToClientPacket::PlayerVotes { voted_for_player }
    }
    pub fn new_PlayerAlive(game: &mut Game)->ToClientPacket{
        let mut alive = Vec::new();
        for player in game.players.iter(){
            alive.push(player.alive);
        }
        ToClientPacket::PlayerAlive { alive }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum RejectJoinReason {
    GameAlreadyStarted,
    RoomFull,
    InvalidRoomCode,
    ServerBusy,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum RejectStartReason {
    GameEndsInstantly,
    ZeroTimeGame,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum GameOverReason {
    ReachedMaxDay,
    /*TODO Winner { who won? }*/
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerButtons{
    pub vote: bool,
    pub target: bool,
    pub day_target: bool,
}
impl PlayerButtons{
    pub fn from_target(game: &Game, actor_index: PlayerIndex, target_index: PlayerIndex)->Self{
        Self{
            vote: actor_index != target_index && game.phase_machine.current_state == PhaseType::Voting && game.get_unchecked_player(actor_index).voting_variables.chosen_vote == None && game.get_unchecked_player(actor_index).alive && game.get_unchecked_player(target_index).alive,
            target: game.get_unchecked_player(actor_index).get_role().can_night_target(actor_index, target_index, game) && game.get_current_phase() == PhaseType::Night,
            day_target: game.get_unchecked_player(actor_index).get_role().can_day_target(actor_index, target_index, game),
        }
    }
    pub fn from(game: &Game, actor_index: PlayerIndex)->Vec<Self>{
        let mut out = Vec::new();
        for target in game.players.iter(){
            out.push(Self::from_target(game, actor_index, target.index));
        }
        out
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum ToServerPacket{
    //Pre Lobby
    Join{room_code: RoomCode},
    Host,

    //Lobby
    SetName{name: String},
    StartGame,
    Kick{player_index: PlayerIndex},
    SetRoleList{role_list: RoleList},
    SetPhaseTime{phase: PhaseType, time: u64},
    SetInvestigatorResults{investigator_results: InvestigatorResults},

    //Game
    Vote{player_index: Option<PlayerIndex>},   //Accusation
    Judgement{verdict: Verdict},  //Vote
    Target{player_index_list: Vec<PlayerIndex>},
    DayTarget{player_index:  PlayerIndex},

    SendMessage{text: String},
    SendWhisper{player_index: PlayerIndex, text: String},
    SaveWill{will: String},
}