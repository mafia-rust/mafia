import { PlayerIndex } from "./gameState.d"
import { Faction } from "./roleListState.d"
import ROLES from "./../resources/roles.json";
import { Doomsayer } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import { AuditorResult } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import { OjoAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import { Hypnotist } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeHypnotistMenu";
import { PuppeteerAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallPuppeteerMenu";
import { KiraGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";

export type RoleState = {
    type: "jailor",
    executionsRemaining: number,
    jailedTargetRef: number | null
} | {
    type: "villager"
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
    type: "snoop",
} | {
    type: "gossip",
} | {
    type: "flowerGirl"
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
    type: "engineer",
    trap: {type: "dismantled"} | {type: "ready"} | {type: "set", target: PlayerIndex, shouldUnset: boolean}
} | {
    type: "armorsmith",
    openShopsRemaining: number,
    // nightOpenShop: boolean,
    // nightProtectedPlayers: Vec<PlayerReference>,
    // playersArmor: Vec<PlayerReference>
} | {
    type: "vigilante",
    state: {type:"notLoaded"} | {type:"willSuicide"} | {type:"loaded",bullets:number} | {type:"suicided"}
} | {
    type: "veteran"
    alertsRemaining: number,
} | {
    type: "marksman"
    state: {type:"notLoaded"} | {type:"shotTownie"} | {type: "marks", marks: PlayerIndex[]}
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
    type: "retrainer"
    backup: PlayerIndex | null,
    retrainsRemaining: number
} | {
    type: "eros"
    action: "loveLink" | "kill"
} | {
    type: "counterfeiter",
    action: "forge" | "noForge",
    fakeRole: Role,
    fakeWill: string
    forgesRemaining: number,
    backup: PlayerIndex | null
} | {
    type: "mafioso"
} | 
(Hypnotist & {type: "hypnotist"})
 | {
    type: "consort"
 } | {
    type: "blackmailer"
} | {
    type: "informant",
} | {
    type: "mortician"
    obscuredPlayers: PlayerIndex[]
} | {
    type: "forger",
    fakeRole: Role,
    fakeWill: string,
    forgesRemaining: number,
    // forgedRef
} | {
    type: "framer"
} | {
    type: "witch"
} | {
    type: "mafiaSupportWildcard"
    role: Role
} | {
    type: "jester"
} | {
    type: "rabbleRouser"
} | 
Doomsayer 
| {
    type: "politician"
} | {
    type: "minion"
} | {
    type: "scarecrow"
} | {
    type: "death",
    souls: number
} | {
    type: "wildcard"
    role: Role
} | {
    type: "trueWildcard"
    role: Role
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
} | {
    type: "arsonist"
} | {
    type: "werewolf",
    trackedPlayers: PlayerIndex[]
} | {
    type: "ojo"
    chosenAction: OjoAction
} | {
    type: "puppeteer"
    action: PuppeteerAction,
    marionettesRemaining: number
} | {
    type: "kira"
    guesses: Record<PlayerIndex, KiraGuess>
} | {
    type: "fiendsWildcard"
    role: Role
} | {
    type: "apostle"
} | {
    type: "disciple"
} | {
    type: "zealot"
}


export type Role = keyof typeof ROLES;
export function getFactionFromRole(role: Role): Faction {
    return ROLES[role].faction as Faction;
}