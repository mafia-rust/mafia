import { Faction, PlayerIndex, Role } from "./gameState.d";

export interface Grave {
    playerIndex: PlayerIndex,

    role: GraveRole,
    deathCause: GraveDeathCause,
    will: String,

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
    faction: Faction
} | {
    type: "suicide"
} | {
    type: "role"
    role: Role
};

export type GravePhase = "day" | "night"