import { PlayerIndex } from "./gameState.d"
import { Faction } from "./roleListState.d"
import ROLES from "./../resources/roles.json";
import { Doomsayer } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import { AuditorResult } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import { OjoAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import { Hypnotist } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeConsortMenu";

export type RoleState = {
    type: "jailor",
    executionsRemaining: number,
    jailedTargetRef: number | null
} | {
    type: "mayor",
    revealed: boolean
} | {
    type: "transporter"
} | {
    type: "detective"
} | {
    type: "lookout"
} | {
    type: "spy"
} | {
    type: "tracker"
} | {
    type: "philosopher"
} | {
    type: "psychic"
} | {
    type: "auditor",
    chosenOutline: number,
    previouslyGivenResults: [number, AuditorResult][]
} | {
    type: "doctor",
    selfHealsRemaining: number,
} | {
    type: "bodyguard",
    selfShieldsRemaining: number,
} | {
    type: "cop",
} | {
    type: "bouncer"
} | {
    type: "engineer"
} | {
    type: "vigilante",
    state: {type:"notLoaded"} | {type:"willSuicide"} | {type:"loaded",bullets:number} | {type:"suicided"}
} | {
    type: "veteran"
    alertsRemaining: number,
} | {
    type: "deputy"
} | {
    type: "escort"
} | {
    type: "medium",
    seancesRemaining: number,
    seancedTarget: PlayerIndex | null
} | {
    type: "retributionist"
} | {
    type: "journalist",
    public: boolean,
    journal: string,
    interviewedTarget: PlayerIndex | null
} | {
    type: "godfather"
    backup: PlayerIndex | null
} | {
    type: "mafioso"
} | 
(Hypnotist & {type: "hypnotist"})
 | {
    type: "blackmailer"
} | {
    type: "informant",
} | {
    type: "janitor"
    cleansRemaining: number,
    // cleanedRef
} | {
    type: "forger",
    fakeRole: Role,
    fakeWill: string,
    forgesRemaining: number,
    // forgedRef
} | {
    type: "witch"
} | {
    type: "mafiaWildCard"
    role: Role
} | {
    type: "jester"
} | {
    type: "hater"
} | 
Doomsayer 
| {
    type: "politician"
} | {
    type: "arsonist"
} | {
    type: "werewolf",
    trackedPlayers: PlayerIndex[]
} | {
    type: "ojo"
    chosenAction: OjoAction
} | {
    type: "death",
    souls: number
} | {
    type: "wildcard"
    role: Role
} | {
    type: "apostle"
} | {
    type: "disciple"
} | {
    type: "zealot"
} | {
    type: "martyr",
    state: {
        type: "won"
    } | {
        type: "leftTown"
    } | {
        type: "stillPlaying",
        bullets: number
    }
}


export type Role = keyof typeof ROLES;
export function getFactionFromRole(role: Role): Faction {
    return ROLES[role].faction as Faction;
}