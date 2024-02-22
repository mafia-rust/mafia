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
    excludedRoles: Role[],
    phaseTimes: PhaseTimes,

    players: Map<PlayerID, LobbyPlayer>,
}
export type LobbyPlayer = {
    name: string,
    host: boolean,
    lostConnection: boolean,
}

type GameState = {
    stateType: "game"
    roomCode: number,

    myIndex: PlayerIndex | null,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    playerOnTrial: PlayerIndex | null,
    phase: Phase | null,
    timeLeftMs: number,
    dayNumber: number,

    roleState: RoleState | null,

    fastForward: boolean,

    will: string,
    notes: string,
    crossedOutOutlines: number[],
    chatFilter: PlayerIndex | null,
    deathNote: string,
    targets: PlayerIndex[],
    voted: PlayerIndex | null,
    judgement: Verdict,
    
    roleList: RoleList,
    excludedRoles: Role[],
    phaseTimes: PhaseTimes

    ticking: boolean
}
export default GameState;

export type PlayerIndex = number;
export type PlayerID = number;
export type Verdict = "innocent"|"guilty"|"abstain";
export type Phase = "morning" | "discussion" | "voting" | "testimony" | "judgement" | "evening" | "night"

export type PhaseTimes = {
    "morning": number,
    "discussion": number,
    "voting": number,
    "testimony": number,
    "judgement": number,
    "evening": number,
    "night": number,
}
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


