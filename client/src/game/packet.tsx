import { PhaseType, PlayerIndex, Verdict, PhaseTimes, Tag, LobbyClientID, ChatGroup, PhaseState, LobbyClient, ModifierType, InsiderGroup } from "./gameState.d"
import { Grave } from "./graveState"
import { ChatMessage } from "../components/ChatMessage"
import { RoleList, RoleOutline } from "./roleListState.d"
import { Role, RoleState } from "./roleState.d"
import { DoomsayerGuess } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/LargeDoomsayerMenu"
import { KiraGuess } from "../menu/game/gameScreenContent/AbilityMenu/AbilitySelectionTypes/KiraSelectionMenu"
import { AbilityInput, ControllerID, SavedController } from "./abilityInput"
import { ListMapData } from "../ListMap"

export type LobbyPreviewData = {
    name: string,
    inGame : boolean,
    players: [LobbyClientID, string][]
}

export type ToClientPacket = {
    type: "pong",
} | {
    type: "rateLimitExceeded",
} | {
    type: "forcedOutsideLobby"
} | {
    type: "forcedDisconnect"
} | {
    type: "lobbyList",
    lobbies: Record<number, LobbyPreviewData>,
} | {
    type: "acceptJoin",
    roomCode: number,
    inGame: boolean,
    playerId: number,
    spectator: boolean
} | {
    type: "rejectJoin",
    reason: string
} | 
// Lobby
{
    type: "yourId",
    playerId: LobbyClientID
} | {
    type: "lobbyClients",
    clients: ListMapData<LobbyClientID, LobbyClient>
} | {
    type: "lobbyName",
    name: string
} | {
    type: "yourPlayerIndex",
    playerIndex: PlayerIndex
} | {
    type: "yourFellowInsiders",
    fellowInsiders: PlayerIndex[]
} | {
    type: "rejectStart",
    reason: string
} | {
    type: "playersHost",
    hosts: LobbyClientID[],
} | {
    type: "playersReady",
    ready: LobbyClientID[],
} | {
    type: "playersLostConnection",
    lostConnection: LobbyClientID[],
} | {
    type: "startGame"
} | {
    type: "gameInitializationComplete"
} | {
    type: "backToLobby",
} | {
    type: "gamePlayers",
    players: string[]
} | {
    type: "roleList",
    roleList: RoleList,
} | {
    type: "roleOutline",
    index: number,
    roleOutline: RoleOutline
} | {
    type: "phaseTime",
    phase: Exclude<PhaseState, { type: "recess" }>, 
    time: number
} | {
    type: "phaseTimes",
    phaseTimeSettings: PhaseTimes
} | {
    type: "enabledRoles",
    roles: Role[]
} | {
    type: "enabledModifiers",
    modifiers: ModifierType[]
} |
// Game
{
    type: "phase",
    phase: PhaseState, 
    dayNumber: number, 
} | {
    type: "phaseTimeLeft",
    secondsLeft: number
} |{
    type: "playerOnTrial",
    playerIndex: PlayerIndex
} | {
    type: "playerAlive", 
    alive: [boolean]
} | {
    type: "playerVotes",
    votesForPlayer: ListMapData<number, number> 
} | {
    type: "yourSendChatGroups",
    sendChatGroups: ChatGroup[]
} | {
    type: "yourInsiderGroups",
    insiderGroups: InsiderGroup[]
} | {
    type: "yourAllowedControllers",
    save: ListMapData<ControllerID, SavedController>,
} | {
    type: "yourRoleLabels",
    roleLabels: ListMapData<PlayerIndex, Role> 
} | {
    type: "yourPlayerTags",
    playerTags: ListMapData<PlayerIndex, Tag[]> 
} | {
    type: "yourWill",
    will: string
} | {
    type: "yourNotes",
    notes: string[]
} | {
    type: "yourCrossedOutOutlines",
    crossedOutOutlines: number[]
} | {
    type: "yourDeathNote", 
    deathNote: string | null
} | {
    type: "yourRoleState",
    roleState: RoleState
} | {
    type: "yourJudgement",
    verdict: Verdict
} | {
    type: "yourVoteFastForwardPhase",
    fastForward: boolean
} | {
    type: "addChatMessages",
    chatMessages: ChatMessage[]
} | {
    type: "nightMessages",
    chatMessages: ChatMessage[]
} | {
    type: "addGrave",
    grave: Grave
} | {
    type: "gameOver",
    reason: string
} | {
    type: "yourPitchforkVote",
    player: PlayerIndex | null
}

export type ToServerPacket = {
    type: "ping",
} | {
    type: "lobbyListRequest",
} | {
    type: "reJoin",
    roomCode: number,
    playerId: number,
} | {
    type: "join", 
    roomCode: number
} | {
    type: "host",
} | {
    type: "kick",
    playerId: number
}
// Lobby
| {
    type: "setSpectator",
    spectator: boolean
} | {
    type: "setName", 
    name: string
} | {
    type: "readyUp", 
    ready: boolean
} | {
    type: "sendLobbyMessage",
    text: string
} | {
    type: "setLobbyName", 
    name: string
} | {
    type: "startGame",
} | {
    type: "setRoleList", 
    roleList: RoleList,
} | {
    type: "setRoleOutline", 
    index: number,
    roleOutline: RoleOutline
} | {
    type: "simplifyRoleList"
} | {
    type: "setPhaseTime", 
    phase: PhaseType, 
    time: number
} | {
    type: "setPhaseTimes", 
    phaseTimeSettings: PhaseTimes
} | {
    type: "setEnabledRoles", 
    roles: Role[], 
} | {
    type: "setEnabledModifiers",
    modifiers: ModifierType[]
} | {
    type: "backToLobby",
} |
// Game
{
    type: "judgement", 
    verdict: Verdict
} | {
    type: "sendChatMessage", 
    text: string,
    block: boolean,
} | {
    type: "sendWhisper", 
    playerIndex: PlayerIndex, 
    text: string
} | {
    type: "saveWill", 
    will: string
} | {
    type: "saveNotes", 
    notes: string[]
} | {
    type: "saveCrossedOutOutlines",
    crossedOutOutlines: number[]
} | {
    type: "saveDeathNote", 
    deathNote: string | null
} | {
    type: "leave",
} | {
    type: "abilityInput",
    abilityInput: AbilityInput
} | {
    type: "setKiraGuess",
    guesses: [PlayerIndex, KiraGuess][]
} | {
    type: "setDoomsayerGuess",
    guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]
} | {
    type: "setConsortOptions",
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
} | {
    type: "voteFastForwardPhase",
    fastForward: boolean
}