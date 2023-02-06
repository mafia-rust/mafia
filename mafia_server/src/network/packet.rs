
enum ToClientPacket {
    PreLobby(PreLobbyToClientPacket),
    Lobby(LobbyToClientPacket),
    Game(GameToClientPacket),
}

enum PreLobbyToClientPacket {
    AcceptJoin,
    AcceptHost,
}

enum LobbyToClientPacket {
    GameStarted,
    PlayerNames,
    Kicked,
    RoleList,
    PhaseTimes,
    InvestigatorResults
}

// All of these are just for synchronizing variables between the 2 so client can see what their doing
enum GameToClientPacket {
    Phase,   // Time remaining in phase
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

enum ToServerPacket {
    PreLobby(PreLobbyToServerPacket),
    Lobby(LobbyToServerPacket),
    Game(GameToServerPacket)
}

enum PreLobbyToServerPacket {
    Join {
        name: String,
    },
    Host {
        name: String,
    }
}

enum LobbyToServerPacket {
    Start,
    Kick,
    RoleList,
    PhaseTimes,
    InvestigatorResults,
}

enum GameToServerPacket {
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    Whisper,
    SendMessage,
    SaveWill,
}
