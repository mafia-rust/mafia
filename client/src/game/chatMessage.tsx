import { Phase, PlayerIndex, Role, Verdict } from "./gameState.d"
import { Grave } from "./grave"

export type ChatMessage = {
    type: "normal", 
    messageSender: MessageSender, 
    text: string, 
    chatGroup: ChatGroup
} | {
    type: "whisper", 
    fromPlayerIndex: PlayerIndex, 
    toPlayerIndex: PlayerIndex, 
    text: string
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
    type: "youDied"
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
    votee: PlayerIndex | null 
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
    targets: PlayerIndex[]
} | 
// Role-specific
{
    type: "mayorRevealed", 
    playerIndex: PlayerIndex
} | {
    type: "mayorCantWhisper"
} | {
    type: "jailedSomeone",
    playerIndex: PlayerIndex
} | {
    type: "jailedTarget"
    playerIndex: PlayerIndex
} | {
    type: "jailorDecideExecute"
    targets: PlayerIndex[]
} | {
    type: "mediumSeance",
    player: PlayerIndex
} | {
    type: "jesterWon"
} | {
    type: "executionerWon"
} | {
    type: "deputyShot",
    shotIndex: PlayerIndex
} | {
    type: "playerWithNecronomicon",
    playerIndex: PlayerIndex
} | {
    type: "roleData", 
    roleData: Role | {
        0: Role
    }
} | {
    type: "roleBlocked", 
    immune : boolean
} | {
    type: "targetSurvivedAttack"
} | {
    type: "youSurvivedAttack"
} |
/* Role-specific */
{
    type: "targetJailed"
} | {
    type: "sheriffResult", 
    suspicious: boolean
} | {
    type: "lookoutResult", 
    players: PlayerIndex[]
} | {
    type: "spyMafiaVisit", 
    players: PlayerIndex[]
} | {
    type: "spyCovenVisit", 
    players: PlayerIndex[]
} | {
    type: "spyBug", 
    bug: String
} | {
    type: "trackerResult",
    players: PlayerIndex[]
} | {
    type: "seerResult",
    enemies: boolean
} | {
    type: "retributionistBug", 
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
    type: "bodyguardProtected"
} | {
    type: "protectedYou"
} | {
    type: "transported"
} | {
    type: "godfatherForcedMafioso"
} | {
    type: "godfatherForcedYou"
} | {
    type: "silenced"
} | {
    type: "framerFramedPlayers", 
    players: PlayerIndex[]
} | {
    type: "playerRoleAndWill", 
    role: Role,
    will: string
} | {
    type: "consigliereResult", 
    role: Role,
    visitedBy: PlayerIndex[],
    visited: PlayerIndex[]
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
