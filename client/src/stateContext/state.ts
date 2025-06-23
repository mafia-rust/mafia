import { ChatMessage } from "../components/ChatMessage";
import ListMap from "../ListMap";
import { LobbyPreviewData } from "./stateType/gameBrowserState";
import { Player, PlayerGameState } from "./stateType/gameState";
import { Grave } from "./stateType/grave";
import { ModifierType } from "./stateType/modifiersState";
import { LobbyClientID, PhaseTimes, PlayerIndex } from "./stateType/otherState";
import { PhaseState } from "./stateType/phaseState";
import { RoleList } from "./stateType/roleListState";
import { Role } from "./stateType/roleState";

export type State = {
    lobbies: Map<number, LobbyPreviewData>,
    
    roomCode: number | null,
    lobbyName: string | null,
    
    client: ListMap<LobbyClientID, ClientObject>,
    myId: number | null,

    roleList: RoleList,
    phaseTimes: PhaseTimes | null,
    enabledRoles: Role[],
    enabledModifiers: ModifierType[],
    
    chatMessages: ChatMessage[],
    graves: Grave[],
    players: Player[],
    phaseState: PhaseState | null,
    timeLeftMs: number | null,
    dayNumber: number,
    missedChatMessages: boolean,
    ticking: boolean,
    initialized: boolean,
    fastForward: boolean,

    clientState: PlayerGameState | {type: "spectator"},
    host: boolean,


    updateChatFilter: (filter: number | null) => void,

    setPrependWhisperFunction: (f: ((index: PlayerIndex) => void)) => void,
    prependWhisper: (index: PlayerIndex) => void
};
export type ClientObject = {
    clientType: ClientObjectType,
    connection: ClientConnection,
    host: boolean,
    index: number | null,
    ready: "host" | "ready" | "notReady",
};
export type ClientObjectType = {
    type: "spectator",
} | {
    type: "player",
    name: string,
};
export type ClientConnection = "connected" | "disconnected" | "couldReconnect";

function createState(): State {
    return {
        lobbies: new Map(),
        
        roomCode: null,
        lobbyName: "Mafia Lobby",
        
        client: new ListMap<LobbyClientID, ClientObject>(),
        myId: null,
    
        roleList: [],
        phaseTimes: null,
        enabledRoles: [],
        enabledModifiers: [],
        
        chatMessages: [],
        graves: [],
        players: [],
        phaseState: null,
        timeLeftMs: null,
        dayNumber: 0,
        missedChatMessages: false,
        ticking: false,
        initialized: false,
        fastForward: false,
    
        clientState: {type: "spectator"},
        host: false,

        updateChatFilter: (filter)=>{},
        setPrependWhisperFunction: (f)=>{},
        prependWhisper: (i)=>{}
    };
}