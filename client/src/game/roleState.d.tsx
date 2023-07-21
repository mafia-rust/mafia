import { PlayerIndex } from "./gameState.d"
import { Faction, FactionAlignment, RoleListEntry, getFactionFromFactionAlignment } from "./roleListState.d"
import ROLES from "./../resources/roles.json";
import { Doomsayer } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";

export type RoleState = {
    role: "jailor",
    executionsRemaining: number,
    jailedTargetRef: number | null
} | {
    role: "mayor",
    revealed: boolean
} | {
    role: "transporter"
} | {
    role: "sheriff"
} | {
    role: "lookout"
} | {
    role: "spy"
} | {
    role: "tracker"
} | {
    role: "seer"
} | {
    role: "doctor",
    selfHealsRemaining: number,
} | {
    role: "bodyguard",
    selfShieldsRemaining: number,
} | {
    role: "vigilante",
    bulletsRemaining: number,
    willSuicide: boolean,
} | {
    role: "veteran"
    alertsRemaining: number,
} | {
    role: "escort"
} | {
    role: "medium",
    seancesRemaining: number,
    seancedTarget: PlayerIndex | null
} | {
    role: "retributionist"
} | {
    role: "mafioso"
} | {
    role: "consort"
} | {
    role: "blackmailer"
} | {
    role: "consigliere",
} | {
    role: "janitor"
    cleansRemaining: number,
} | {
    role: "witch"
} | {
    role: "jester"
} | {
    role: "executioner"
} | 
Doomsayer 
| {
    role: "vampire"
} | {
    role: "amnesiac"
    roleListEntry: RoleListEntry
}


export type Role = keyof typeof ROLES;
export function getFactionFromRole(role: Role): Faction {
    return getFactionFromFactionAlignment(getFactionAlignmentFromRole(role));
}
export function getFactionAlignmentFromRole(role: Role): FactionAlignment {
    return ROLES[role as keyof typeof ROLES].factionAlignment as FactionAlignment;
}