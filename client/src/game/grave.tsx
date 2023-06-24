import { PlayerIndex } from "./gameState.d";
import { Faction } from "./roleListState.d";
import { Role } from "./roleState.d";

export interface Grave {
    playerIndex: PlayerIndex,

    role: GraveRole,
    deathCause: GraveDeathCause,
    will: string,

    diedPhase: GravePhase,
    dayNumber: number,
}

export type GraveRole = {
    type: "cleaned" | "petrified"
} |  {
    type: "role"
    role: Role
};
export type GraveDeathCause = {
    type: "lynching"
} | {
    type: "killers"
    killers: GraveKiller[]
};
export type GraveKiller = {
    type: "faction"
    value: Faction
} | {
    type: "suicide"
} | {
    type: "role"
    value: Role
};

export type GravePhase = "day" | "night"