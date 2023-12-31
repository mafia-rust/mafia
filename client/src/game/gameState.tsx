import GameState, { LobbyPlayer, LobbyState, Player, PlayerID } from "./gameState.d"


export function createLobbyState(): LobbyState {
    return {
        stateType: "lobby",
        roomCode: "",

        myId: null,

        roleList: [],
        excludedRoles: [],
        phaseTimes: {
            morning: 5,
            discussion: 45, 
            voting: 30, 
            testimony: 20, 
            judgement: 20, 
            evening: 7, 
            night: 37,
        },

        players: new Map<PlayerID, LobbyPlayer>(),
    }
}

export function createGameState(): GameState {
    return {
        stateType: "game",
        roomCode: "",

        myIndex: null,

        chatMessages : [],
        graves: [],
        players: [],
        
        playerOnTrial: null,
        phase: null,
        timeLeftMs: 0,
        dayNumber: 1,

        roleState: null,

        will: "",
        notes: "",
        deathNote: "",
        targets: [],
        voted: null,
        judgement: "abstain",
        
        roleList: [],
        excludedRoles: [],
        phaseTimes: {
            morning: 5,
            discussion: 45, 
            voting: 30, 
            testimony: 20, 
            judgement: 20, 
            evening: 7, 
            night: 37,
        },

        ticking: true,
    }
}

export function createPlayer(name: string, index: number): Player {
    return{
        name: name,
        index: index,
        buttons: {
            dayTarget: false,
            target: false,
            vote: false,
        },
        numVoted: 0,
        alive: true,
        roleLabel: null,
        playerTags: [],
        host: false,

        toString() {
            return ""+(this.index+1)+": " + this.name;
        }
    }
}


