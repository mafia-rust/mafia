import ListMap from "../ListMap"
import GameState, { LobbyClient, LobbyState, PhaseTimes, Player, LobbyClientID, PlayerGameState } from "./gameState.d"


export function defaultPhaseTimes(): PhaseTimes {
    return {
        briefing: 45,
        obituary: 60,
        discussion: 120,
        nomination: 120,
        testimony: 30,
        judgement: 60,
        finalWords: 30,
        dusk: 30,
        night: 60,
    }
}

export function createLobbyState(): LobbyState {
    return {
        stateType: "lobby",
        roomCode: 0,
        lobbyName: "Mafia Lobby",

        myId: null,

        roleList: [],
        phaseTimes: defaultPhaseTimes(),
        enabledRoles: [],
        enabledModifiers: [],

        players: new ListMap<LobbyClientID, LobbyClient>(),
        chatMessages: [],
    }
}

export function createGameState(): GameState {
    return {
        stateType: "game",
        roomCode: 0,
        lobbyName: "",

        initialized: false,

        myId: null,

        chatMessages : [],
        graves: [],
        players: [],
        
        phaseState: {type:"briefing"},
        timeLeftMs: 0,
        dayNumber: 1,

        fastForward: false,
        
        roleList: [],
        enabledRoles: [],
        phaseTimes: defaultPhaseTimes(),
        enabledModifiers: [],

        ticking: true,

        clientState: createPlayerGameState(),
        host: false,

        missedChatMessages: false
    }
}

export function createPlayerGameState(): PlayerGameState {
    return {
        type: "player",

        myIndex: 0,
        
        roleState: { type: "detective" },

        savedControllers: [],

        will: "",
        notes: [],
        crossedOutOutlines: [],
        chatFilter: null,
        deathNote: "",
        targets: [],
        voted: null,
        judgement: "abstain",

        fellowInsiders: [],

        sendChatGroups: [],
        insiderGroups: [],

        missedWhispers: []
    }
}

export function createPlayer(name: string, index: number): Player {
    return{
        name: name,
        index: index,
        buttons: {
            vote: false,
        },
        numVoted: 0,
        alive: true,
        roleLabel: null,
        playerTags: [],

        toString() {
            return ""+(this.index+1)+": " + this.name;
        }
    }
}


