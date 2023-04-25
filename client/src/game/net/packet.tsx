import { Grave, Phase, PlayerIndex, Role, RoleListEntry, Verdict } from "../gameState.d"

export type ToClientPacket = {
    type: "acceptJoin",
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
    type: "phaseTime",
    phase: Phase, 
    time: number
} | {
    type: "investigatorResults",
    investigatorResults: Role[][]
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
    type: "playerButtons", 
    buttons: [{
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    }]
} | {
    type: "playerAlive", 
    alive: [boolean]
} | {
    type: "playerVotes",
    voted_for_player: [number]
} | {
    type: "yourWill",
    will: string
} | {
    type: "yourRole",
    role: {
        0: Role
        /* OTHER FIELDS TODO */
    }
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
    chatMessages: [any]
} | {
    type: "addGrave",
    grave: Grave
} | {
    type: "gameOver",
    reason: string /* TODO GameOverReason */
}

export type ToServerPacket = {
    type: "join", 
    roomCode: number
} | {
    type: "host",
}
//Lobby
| {
    type: "setName", 
    name: String
} | {
    type: "startGame",
} | {
    type: "kick", 
    playerIndex: PlayerIndex
} | {
    type: "setRoleList", 
    roleList: RoleListEntry[]
} | {
    type: "setPhaseTime", 
    phase: Phase, 
    time: number
} | {
    type: "setInvestigatorResults", 
    investigatorResults: Role[][]
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
    text: String
} | {
    type: "sendWhisper", 
    playerIndex: PlayerIndex, 
    text: String
} | {
    type: "saveWill", 
    will: String
}