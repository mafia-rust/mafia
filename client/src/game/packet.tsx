import { Phase, PlayerIndex, Verdict, PhaseTimes, Tag } from "./gameState.d"
import { Grave } from "./grave"
import { ChatMessage } from "../components/ChatMessage"
import { RoleListEntry } from "./roleListState.d"
import { Role, RoleState } from "./roleState.d"
import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu"

export type ToClientPacket = {
    type: "acceptJoin",
    inGame: boolean
} | {
    type: "rejectJoin",
    reason: string /* TODO RejectJoinReason */
} | {
    type: "acceptHost",
    roomCode: number /* TODO RoomCode */
} |
// Lobby
{
    type: "yourName",
    name: string
} | {
    type: "yourPlayerIndex",
    playerIndex: PlayerIndex
} | {
    type: "players",
    names: [string]
} | {
    type: "kicked"
} | {
    type: "rejectStart",
    reason: string /* TODO RejectStartReason */
} | {
    type: "startGame"
} | {
    type: "roleList",
    roleList: RoleListEntry[]
} | {
    type: "roleListEntry",
    index: number,
    roleListEntry: RoleListEntry
} | {
    type: "phaseTime",
    phase: Phase, 
    time: number
} | {
    type: "phaseTimes",
    phaseTimeSettings: PhaseTimes
} | {
    type: "excludedRoles",
    roles: RoleListEntry[]
} | {
    type: "youAreHost"
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
    votedForPlayer: Map<PlayerIndex, number>
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
    type: "yourRole",
    role: Role
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
    //////////////////////////////////////////////////////////////////////////////////////////////////
export type ToServerPacket = {
    type: "join", 
    roomCode: number
} | {
    type: "host",
}
//Lobby
| {
    type: "setName", 
    name: string
} | {
    type: "startGame",
} | {
    type: "kick", 
    playerIndex: PlayerIndex
} | {
    type: "setRoleList", 
    roleList: RoleListEntry[]
} | {
    type: "setRoleListEntry", 
    index: number,
    roleListEntry: RoleListEntry
} | {
    type: "setPhaseTime", 
    phase: Phase, 
    time: number
} | {
    type: "setPhaseTimes", 
    phaseTimeSettings: PhaseTimes
} | {
    type: "setExcludedRoles", 
    roles: RoleListEntry[], 
} | 
//Game
{ //Accusation
    type: "vote", 
    playerIndex: PlayerIndex | null
} |
{ //Vote
    type: "judgement", 
    verdict: Verdict
} |
{
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
}