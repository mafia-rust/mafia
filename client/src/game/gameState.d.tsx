import { Grave } from "./graveState";
import { ChatMessage } from "../components/ChatMessage";
import { Role, RoleState } from "./roleState.d";
import { RoleList } from "./roleListState.d";
import { LobbyPreviewData } from "./packet";


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
    phaseTimes: PhaseTimes,
    enabledRoles: Role[],
    enabledModifiers: ModifierType[],

    players: Map<LobbyClientID, LobbyClient>,
    chatMessages: ChatMessage[],
}
export type LobbyClient = {
    host: boolean,
    connection: "connected" | "disconnected" | "couldReconnect",
    clientType: LobbyClientType
}
export type LobbyClientType = {
    type: "spectator"
} | PlayerClientType;
export type PlayerClientType = {
    type: "player",
    name: string,
}

type GameState = {
    stateType: "game"
    roomCode: number,
    lobbyName: string,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    phaseState: PhaseState,
    timeLeftMs: number,
    dayNumber: number,

    fastForward: boolean,
    
    roleList: RoleList,
    enabledRoles: Role[],
    phaseTimes: PhaseTimes,
    enabledModifiers: ModifierType[],

    ticking: boolean,

    clientState: PlayerGameState | {type: "spectator"},
    host: boolean,

}
export default GameState;

export type PlayerGameState = {
    type: "player",

    myIndex: PlayerIndex,
    
    roleState: RoleState,

    will: string,
    notes: string,
    crossedOutOutlines: number[],
    chatFilter: PlayerIndex | null,
    deathNote: string,
    voted: PlayerIndex | null,
    judgement: Verdict,
    
    forfeitVote: boolean,
    pitchforkVote: PlayerIndex | null,

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

export type ChatGroup = "all" | "dead" | "mafia" | "cult" | "jail" | "interview" | "puppeteer";

export type PhaseTimes = Record<PhaseType, number>;

export type Tag = | "godfatherBackup" | "werewolfTracked" | "doused" | "revolutionaryTarget" | "morticianTagged" | "puppeteerMarionette" | "loveLinked" | "forfeitVote";

export const MODIFIERS = ["obscuredGraves", "randomLoveLinks", "deadCanChat", "noAbstaining", "noDeathCause"] as const;
export type ModifierType = (typeof MODIFIERS)[number];

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
    playerTags: Tag[]

    toString(): string
}


