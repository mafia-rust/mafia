import GameState, { LobbyState, Player } from "./gameState.d"


export function createLobbyState(): LobbyState {
    return {
        stateType: "lobby",

        myName: null,
        host: false,

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

        players: [],//new Map<PlayerID, LobbyPlayer>(),
    }
}

export function createGameState(): GameState {
    return {
        stateType: "game",

        myName: null,
        myIndex: null,
        host: false,

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

        still_ticking: true,
    }
}

export function createPlayer(name: string, index: number, id: number): Player {
    return{
        name: name,
        index: index,
        id: id,
        buttons: {
            dayTarget: false,
            target: false,
            vote: false,
        },
        numVoted: 0,
        alive: true,
        roleLabel: null,
        playerTags: [],

        toString() {
            return ""+(this.index+1)+"-" + this.name;
        }
    }
}


