import { PlayerIndex } from "./gameState.d";
import { Faction } from "./roleListState.d";
import { Role } from "./roleState.d";

export type Grave = {
    playerIndex: PlayerIndex,

    role: GraveRole,
    deathCause: GraveDeathCause,
    will: string,
    deathNotes: string[],

    diedPhase: GravePhase,
    dayNumber: number,
}

export type GraveRole = {
    type: "cleaned"
} |  {
    type: "role"
    role: Role
};
export type GraveDeathCause = {
    type: "lynching" | "leftTown"
} | {
    type: "killers"
    killers: GraveKiller[]
}
export type GraveKiller = {
    type: "faction"
    value: Faction
} | {
    type: "suicide"
} | {
    type: "quit"
} | {
    type: "role"
    value: Role
};

export type GravePhase = "day" | "night"