import { Phase, PlayerIndex, Role, Verdict } from "./gameState.d"
import { Grave } from "./grave"

export type ChatMessage = {
    type: "normal", 
    messageSender: MessageSender, 
    text: String, 
    chatGroup: ChatGroup
} | {
    type: "whisper", 
    fromPlayerIndex: PlayerIndex, 
    toPlayerIndex: PlayerIndex, 
    text: String
} | {
    type: "broadcastWhisper", 
    whisperer: PlayerIndex, 
    whisperee: PlayerIndex 
} | 
// System
{
    type: "roleAssignment", 
    role: Role
} | {
    type: "playerDied", 
    grave: Grave
} | {
    type: "gameOver"
} | {
    type: "phaseChange", 
    phase: Phase, 
    dayNumber: number
} | 
// Trial
{
    type: "trialInformation", 
    requiredVotes: number, 
    trialsLeft: number
} | {
    type: "voted", 
    voter: PlayerIndex, 
    votee: PlayerIndex | undefined 
} | {
    type: "playerOnTrial", 
    playerIndex: PlayerIndex
} | {
    type: "judgementVote", 
    voterPlayerIndex: PlayerIndex
} | {
    type: "judgementVerdict", 
    voterPlayerIndex: PlayerIndex, 
    verdict: Verdict
} | {
    type: "trialVerdict", 
    playerOnTrial: PlayerIndex, 
    innocent: number, 
    guilty: number
} | 
// Misc.
{
    type: "targeted", 
    targeter: PlayerIndex, 
    target: PlayerIndex | undefined
} | {
    type: "nightInformation", 
    nightInformation: NightInformation 
} | 
// Role-specific
{
    type: "mayorRevealed", 
    playerIndex: PlayerIndex
} | {
    type: "mayorCantWhisper"
} | {
    type: "jailed"
} | {
    type: "jailorDecideExecuteYou"
} | {
    type: "mediumSeanceYou"
} | {
    type: "jesterWon"
} | {
    type: "executionerWon"
} | {
    type: "playerWithNecronomicon",
    playerIndex: PlayerIndex
} | {
    type: "roleData", 
    roleData: Role | {
        0: Role
    }
}

export type MessageSender = {
    type: "player", 
    player: PlayerIndex
} | {
    type: "jailor"
} | {
    type: "medium"
}

export type ChatGroup =
    | "all"
    | "mafia"
    | "dead"
    | "vampire"
    | "coven"

export type NightInformation = {
    type: "roleBlocked", 
    immune : boolean
} | {
    type: "targetSurvivedAttack"
} | {
    type: "youSurvivedAttack"
} | {
    type: "youDied"
} |
/* Role-specific */
{
    type: "spyMafiaVisit", 
    players: PlayerIndex[]
} | {
    type: "spyBug", 
    message: ChatMessage
} | {
    type: "veteranAttackedYou"
} | {
    type: "veteranAttackedVisitor"
} | {
    type: "vigilanteSuicide"
} | {
    type: "doctorHealed"
} | {
    type: "doctorHealedYou"
} | {
    type: "bodyguardProtected"
} | {
    type: "bodyguardProtectedYou"
} | {
    type: "transported"
} | {
    type: "godfatherForcedMafioso"
} | {
    type: "godfatherForcedYou"
} | {
    type: "blackmailed"
} | {
    type: "framerFramedPlayers", 
    players: PlayerIndex[]
} | {
    type: "janitorResult", 
    role: Role, 
    will: String 
} | {
    type: "forgerResult", 
    role: Role, 
    will: String 
} | {
    type: "consigliereResult", 
    role: Role 
} | {
    type: "sheriffResult", 
    suspicious: boolean
} | {
    type: "lookoutResult", 
    players: PlayerIndex[]
} | {
    type: "investigatorResult", 
    roles: Role[]
} | {
    type: "witchTargetImmune"
} | {
    type: "witchedYou", 
    immune: boolean
} | {
    type: "witchBug", 
    message: ChatMessage
} | {
    type: "arsonistCleanedSelf"
} | {
    type: "arsonistDousedPlayers", 
    players: PlayerIndex[]
}