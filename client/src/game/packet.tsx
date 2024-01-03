import { Phase, PlayerIndex, Verdict, PhaseTimes, Tag, PlayerID } from "./gameState.d"
import { Grave } from "./grave"
import { ChatMessage } from "../components/ChatMessage"
import { RoleOutline } from "./roleListState.d"
import { Role, RoleState } from "./roleState.d"
import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu"

export type ToClientPacket = 
{
    type: "rateLimitExceeded",
} | {
    type: "lobbyList",
    roomCodes: number[],
} | {
    type: "acceptJoin",
    roomCode: number,
    inGame: boolean,
    playerId: number,
} | {
    type: "rejectJoin",
    reason: string /* TODO RejectJoinReason */
} | 
// Lobby
{
    type: "yourId",
    playerId: PlayerID
} | {
    type: "lobbyPlayers",
    players: Map<PlayerID, string>
} | {
    type: "yourPlayerIndex",
    playerIndex: PlayerIndex
} | {
    type: "rejectStart",
    reason: string /* TODO RejectStartReason */
} | {
    type: "playersHost",
    hosts: PlayerID[],
} | {
    type: "playersLostConnection",
    lostConnection: PlayerID[],
} | {
    type: "startGame"
} | {
    type: "gamePlayers",
    players: string[]
} | {
    type: "roleList",
    roleList: RoleOutline[]
} | {
    type: "roleOutline",
    index: number,
    roleOutline: RoleOutline
} | {
    type: "phaseTime",
    phase: Phase, 
    time: number
} | {
    type: "phaseTimes",
    phaseTimeSettings: PhaseTimes
} | {
    type: "excludedRoles",
    roles: RoleOutline[]
} | 
// Game
{
    type: "phase",
    phase: Phase, 
    dayNumber: number, 
    secondsLeft: number
} | {
    type: "playerOnTrial",
    playerIndex: PlayerIndex
} | {
    type: "playerAlive", 
    alive: [boolean]
} | {
    type: "playerVotes",
    votesForPlayer: Map<PlayerIndex, number>
} | {
    type: "yourButtons", 
    buttons: [{
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    }]
} | {
    type: "yourRoleLabels",
    roleLabels: Map<PlayerIndex, Role>
} | {
    type: "yourPlayerTags",
    playerTags: Map<PlayerIndex, Tag[]>
} | {
    type: "yourWill",
    will: string
} | {
    type: "yourNotes",
    notes: string
} | {
    type: "yourDeathNote", 
    deathNote: string | null
} | {
    type: "yourRoleState",
    roleState: RoleState
} | {
    type: "yourTarget",
    playerIndices: [PlayerIndex]
} | {
    type: "yourVoting",
    playerIndex: PlayerIndex | null
} | {
    type: "yourJudgement",
    verdict: Verdict
} | {
    type: "addChatMessages",
    chatMessages: ChatMessage[]
} | {
    type: "addGrave",
    grave: Grave
} | {
    type: "gameOver",
    reason: string /* TODO GameOverReason */
}

export type ToServerPacket = {
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
}
// Lobby
| {
    type: "setName", 
    name: string
} | {
    type: "startGame",
} | {
    type: "setRoleList", 
    roleList: RoleOutline[]
} | {
    type: "setRoleOutline", 
    index: number,
    roleOutline: RoleOutline
} | {
    type: "setPhaseTime", 
    phase: Phase, 
    time: number
} | {
    type: "setPhaseTimes", 
    phaseTimeSettings: PhaseTimes
} | {
    type: "setExcludedRoles", 
    roles: RoleOutline[], 
} | 
// Game
{
    type: "vote", 
    playerIndex: PlayerIndex | null
} | {
    type: "judgement", 
    verdict: Verdict
} | {
    type: "target", 
    playerIndexList: PlayerIndex[]
} | {
    type: "dayTarget", 
    playerIndex:  PlayerIndex
} | {
    type: "sendMessage", 
    text: string
} | {
    type: "sendWhisper", 
    playerIndex: PlayerIndex, 
    text: string
} | {
    type: "saveWill", 
    will: string
} | {
    type: "saveNotes", 
    notes: string
} | {
    type: "saveDeathNote", 
    deathNote: string | null
} | {
    type: "leave",
} | {
    type: "setForgerWill",
    role: Role,
    will: string
} | {
    type: "setDoomsayerGuess",
    guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]
} | {
    type: "setAmnesiacRoleOutline",
    roleOutline: RoleOutline
}