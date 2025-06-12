import { ChatMessage } from "../../components/ChatMessage";
import { RoleList } from "./roleListState"
import ListMap, { ListMapData } from "../../ListMap"
import { Grave } from "./grave";
import { ChatGroup, ClientConnection, defaultPhaseTimes, InsiderGroup, LobbyClientID, PhaseTimes, PlayerIndex, Verdict } from "./otherState";
import { PhaseState } from "./phaseState";
import { Role, RoleState } from "./roleState";
import { ModifierType } from "./modifiersState";
import { ChatFilter } from "../../menu/game/gameScreenContent/ChatMenu";
import { ControllerID, SavedController } from "../../game/abilityInput";
import { Tag } from "./tagState";

export type GameState = {
    type: "game",
    roomCode: number,
    lobbyName: string,
    
    initialized: boolean,

    myId: number,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    phaseState: PhaseState,
    timeLeftMs: number | null,
    dayNumber: number,

    fastForward: boolean,
    
    roleList: RoleList,
    enabledRoles: Role[],
    phaseTimes: PhaseTimes,
    enabledModifiers: ModifierType[],

    ticking: boolean,

    clientState: PlayerGameState | {type: "spectator"},
    host: null | {
        clients: ListMap<LobbyClientID, GameClient>
    },

    missedChatMessages: boolean,

    updateChatFilter: (filter: number | null) => void,

    setPrependWhisperFunction: (f: ((index: PlayerIndex) => void)) => void,
    prependWhisper: (index: PlayerIndex) => void
}
export function createGameState(spectator: boolean): GameState {
    const gameState: GameState = {
        type: "game",
        roomCode: 0,
        lobbyName: "",

        initialized: false,

        myId: -1,

        chatMessages : [],
        graves: [],
        players: [],
        
        phaseState: { type:"briefing" },
        timeLeftMs: 0,
        dayNumber: 1,

        fastForward: false,
        
        roleList: [],
        enabledRoles: [],
        phaseTimes: defaultPhaseTimes(),
        enabledModifiers: [],

        ticking: true,

        clientState: spectator?{type: "spectator"}:createPlayerGameState(),
        host: null,

        missedChatMessages: false,

        updateChatFilter: (filter: number | null) => {
            if(gameState.clientState.type === "player"){
                gameState.clientState.chatFilter = filter===null?null:{
                    type: "playerNameInMessage",
                    player: filter
                };
            }},
        setPrependWhisperFunction: (f) => {
            gameState.prependWhisper = f;
        },
        prependWhisper: _ => {}
    }

    return gameState;
}


export type PlayerGameState = {
    type: "player",

    myIndex: PlayerIndex,
    
    roleState: RoleState,

    will: string,
    notes: string[],
    crossedOutOutlines: number[],
    chatFilter: ChatFilter,
    deathNote: string,
    judgement: Verdict,

    savedControllers: ListMapData<ControllerID, SavedController>,

    fellowInsiders: PlayerIndex[],

    sendChatGroups: ChatGroup[],
    insiderGroups: InsiderGroup[],
    
    missedWhispers: PlayerIndex[],

    updateChatFilter: (filter: PlayerIndex | null)=>void
}
function createPlayerGameState(): PlayerGameState {
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
        judgement: "abstain",

        fellowInsiders: [],

        sendChatGroups: [],
        insiderGroups: [],

        missedWhispers: [],

        updateChatFilter(filter) {
            
        },
    }
}


export type Player = {
    name: string,
    index: number,
    numVoted: number,
    alive: boolean,
    roleLabel: Role | null,
    playerTags: Tag[]

    toString(): string
}
function createPlayer(name: string, index: number): Player {
    return{
        name: name,
        index: index,
        numVoted: 0,
        alive: true,
        roleLabel: null,
        playerTags: [],

        toString() {
            return ""+(this.index+1)+": " + this.name;
        }
    }
}

export type GameClient = {
    clientType: GameClientType,
    connection: ClientConnection,
    host: boolean,
}
export type GameClientType = {
    type: "spectator",
    index: number
} | {
    type: "player",
    index: number,
}