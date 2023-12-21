import { PlayerIndex } from "./gameState.d"
import { Faction, FactionAlignment, RoleOutline, getFactionFromFactionAlignment } from "./roleListState.d"
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
    role: "psychic"
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
    role: "politician"
} | {
    role: "death",
    souls: number
} | {
    role: "vampire"
} | {
    role: "amnesiac"
    roleOutline: RoleOutline
}


export type Role = keyof typeof ROLES;
export function getFactionFromRole(role: Role): Faction {
    return getFactionFromFactionAlignment(getFactionAlignmentFromRole(role));
}
export function getFactionAlignmentFromRole(role: Role): FactionAlignment {
    return ROLES[role as keyof typeof ROLES].factionAlignment as FactionAlignment;
}