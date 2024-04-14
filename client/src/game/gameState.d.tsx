import { Grave } from "./graveState";
import { ChatMessage } from "../components/ChatMessage";
import { Role, RoleState } from "./roleState.d";
import { RoleList } from "./roleListState.d";
import { LobbyPreviewData } from "./packet";
import GAME_MANAGER from "..";


export type State = Disconnected | OutsideLobbyState | LobbyState | GameState;

export type Disconnected = {
    stateType: "disconnected"
}

export type OutsideLobbyState = {
    stateType: "outsideLobby",

    selectedRoomCode: string | null,
    lobbies: Map<number, LobbyPreviewData>,
}


//Change this to use PlayerID for player map and playerID for who I AM instead of myName and host
export type LobbyState = {
    stateType: "lobby"
    roomCode: number,
    lobbyName: string,

    myId: number | null,

    roleList: RoleList,
    excludedRoles: Role[],
    phaseTimes: PhaseTimes,

    players: Map<LobbyClientID, LobbyClient>,
}
export type LobbyClient = {
    host: boolean,
    connection: "connected" | "disconnected" | "couldReconnect",
    clientType: LobbyClientType
}
export type LobbyClientType = {
    type: "spectator"
} | {
    type: "player",
    name: string,
}

type GameState = {
    stateType: "game"
    roomCode: number,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    phaseState: PhaseState,
    timeLeftMs: number,
    dayNumber: number,

    fastForward: boolean,
    
    roleList: RoleList,
    excludedRoles: Role[],
    phaseTimes: PhaseTimes

    ticking: boolean,

    clientState: PlayerGameState | {type: "spectator"},

}
export default GameState;

export type PlayerGameState = {
    type: "player",

    myIndex: PlayerIndex | null,
    
    roleState: RoleState | null,

    will: string,
    notes: string,
    crossedOutOutlines: number[],
    chatFilter: PlayerIndex | null,
    deathNote: string,
    targets: PlayerIndex[],
    voted: PlayerIndex | null,
    judgement: Verdict,

    sendChatGroups: ChatGroup[],
}

export type PlayerIndex = number;
export type LobbyClientID = number;
export type Verdict = "innocent"|"guilty"|"abstain";
export const PHASES = ["briefing", "obituary", "discussion", "nomination", "testimony", "judgement", "finalWords", "dusk", "night"] as const;
export type PhaseType = (typeof PHASES)[number];
export type PhaseState = {type: "briefing"} | {type: "dusk"} | {type: "night"} | {type: "obituary"} | {type: "discussion"} | 
{
    type: "nomination",
    trialsLeft: number
} | {
    type: "testimony",
    playerOnTrial: PlayerIndex
    trialsLeft: number
} | {
    type: "judgement",
    playerOnTrial: PlayerIndex
    trialsLeft: number
} | {
    type: "finalWords",
    playerOnTrial: PlayerIndex
}

export type ChatGroup = "all" | "dead" | "mafia" | "cult" | "jail" | "interview";

export type PhaseTimes = Record<PhaseType, number>;

export type Tag =
| "godfatherBackup"
| "werewolfTracked"
| "doused"
| "hexed"
| "necronomicon"
| "executionerTarget"
| "insane"

export type Player = {
    name: string,
    index: number
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    },
    numVoted: number,
    alive: boolean,
    roleLabel: Role | null,
    playerTags: Tag[],
    host: boolean,

    toString(): string
}


