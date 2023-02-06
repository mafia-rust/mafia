
/*
enum ToClientPacket{
    PreLobby(PreLobbyToClientPacket),
    Lobby(LobbyToClientPacket),
    Game(GameToClientPacket),
}
enum PreLobbyToClientPacket{
    AcceptJoin,
    AcceptHost,
}
enum LobbyToClientPacket{
    GameStarted,
    PlayerNames,
    Kicked,
    RoleList,
    PhaseTimes,
    InvestigatorResults
}
enum GameToClientPacket{


    ////////All of these are just for syncronizing variables between the 2 so client can see what their doing
    Phase,   //how much time is left with this
    PlayerOnTrial,

    NewChatMessage,

    YourTarget,
    YourVoting,
    YourJudgement,
    YourWhispering,
    YourRole,
    YourWill,

    ChatGroups,

    RoleList,
    InvestigatorResults,

    Players,
    PlayerButtons,

    //a way to syncronise the entire game for someone who joined late
}




enum ToServerPacket{
    PreLobby(PreLobbyToServerPacket),
    Lobby(LobbyToServerPacket),
    Game(GameToServerPacket)
}
enum PreLobbyToServerPacket{
    Join{
        name: String,
    },
    Host{
        name: String,
    }
}
enum LobbyToServerPacket{
    Start,
    Kick,
    RoleList,
    PhaseTimes,
    InvestigatorResults,
}
enum GameToServerPacket{
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    Whisper,
    SendMessage,
    SaveWill,
}
*/

/*
Examples of how these will look in json, maybe

List of packets
{
    [
        {
            type: "LobbyToServer",
            sub_type: "Kick"
            packet: {
                PlayerID: 7
            }
        },
        {
            GameToServer: {
                DayTarget: {
                    PlayerID: 2
                }
            }
        }
    ]
}
*/