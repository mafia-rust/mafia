import { Grave } from "./grave";
import { ChatMessage } from "../components/ChatMessage";
import { Role, RoleState } from "./roleState.d";
import { RoleOutline } from "./roleListState.d";

export default interface GameState {
    inGame: boolean;

    myName: string | null,
    myIndex: PlayerIndex | null,
    host: boolean,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    playerOnTrial: PlayerIndex | null,
    phase: Phase | null,
    timeLeftMs: number,
    dayNumber: number,

    roleState: RoleState | null,

    will: string,
    notes: string,
    deathNote: string,
    targets: PlayerIndex[],
    voted: PlayerIndex | null,
    judgement: Verdict,
    
    roleList: RoleOutline[],
    excludedRoles: RoleOutline[],
    phaseTimes: PhaseTimes
}

export type PlayerIndex = number;
export type PlayerID = number;
export type Verdict = "innocent"|"guilty"|"abstain";
export type Phase = "morning" | "discussion" | "voting" | "testimony" | "judgement" | "evening" | "night"

export interface PhaseTimes {
    "morning": number,
    "discussion": number,
    "voting": number,
    "testimony": number,
    "judgement": number,
    "evening": number,
    "night": number,
}
export type Tag =
| "doused"
| "hexed"
| "necronomicon"
| "executionerTarget"
| "insane"

export interface Player {
    name: string,
    index: number
    id: number,
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    },
    numVoted: number,
    alive: boolean,
    roleLabel: Role | null,
    playerTags: Tag[],

    toString(): string
}


