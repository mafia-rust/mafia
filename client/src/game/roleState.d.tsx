import { PlayerIndex } from "./gameState.d"
import { RoleSet } from "./roleListState.d"
import ROLES from "./../resources/roles.json";
import { ChatMessageVariant } from "../components/ChatMessage";
import { AuditorResult } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/AuditorMenu";
import { RecruiterAction } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/RecruiterMenu";
import { Hypnotist } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/LargeHypnotistMenu";
import { Doomsayer } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/LargeDoomsayerMenu";
import { PuppeteerAction } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/SmallPuppeteerMenu";
import { KiraGuess } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/KiraMenu";
import { TwoRoleOptionInput } from "./abilityInput";

export type RoleState = {
    type: "jailor",
    executionsRemaining: number,
    jailedTargetRef: number | null
} | {
    type: "villager"
} | {
    type: "mayor"
} | {
    type: "transporter"
} | {
    type: "detective"
} | {
    type: "lookout"
} | {
    type: "spy"
} | {
    type: "pyrolisk"
} | {
    type: "tracker"
} | {
    type: "philosopher"
} | {
    type: "psychic"
} | {
    type: "auditor",
    chosenOutline: [number | null, number | null],
    previouslyGivenResults: [number, AuditorResult][]
} | {
    type: "snoop",
} | {
    type: "gossip",
} | {
    type: "tallyClerk"
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
    trap: {type: "dismantled"} | {type: "ready"} | {type: "set"}
} | {
    type: "armorsmith",
    openShopsRemaining: number,
} | {
    type: "steward",
    stewardProtectsRemaining: number,
    roleChosen: TwoRoleOptionInput,
    previousRoleChosen: TwoRoleOptionInput
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
    type: "rabblerouser"
} | {
    type: "escort"
} | {
    type: "medium",
    seancesRemaining: number,
    seancedTarget: PlayerIndex | null
} | {
    type: "retributionist"
} | {
    type: "reporter",
    public: boolean,
    report: string,
    interviewedTarget: PlayerIndex | null
} | {
    type: "godfather"
    backup: PlayerIndex | null
} | {
    type: "impostor"
    backup: PlayerIndex | null,
    fakeRole: Role
} | {
    type: "counterfeiter",
    action: "forge" | "noForge",
    fakeRole: Role,
    fakeWill: string
    forgesRemaining: number,
    backup: PlayerIndex | null
} | {
    type: "recruiter",
    action: RecruiterAction,
    recruitsRemaining: number
    backup: PlayerIndex | null
} | {
    type: "mafioso"
} | {
    type: "mafiaKillingWildcard"
    role: Role
} | {
    type: "goon"
} |
(Hypnotist & {type: "hypnotist"})
 | {
    type: "consort"
 } | {
    type: "blackmailer"
} | {
    type: "informant",
} | {
    type: "mortician",
    obscuredPlayers: PlayerIndex[],
    cremationsRemaining: number
} | {
    type: "forger",
    fakeRole: Role,
    fakeWill: string,
    forgesRemaining: number,
} | {
    type: "disguiser",
    currentTarget: PlayerIndex | null,
    disguisedRole: Role,
} | {
    type: "framer"
} | {
    type: "mafiaWitch"
} | {
    type: "necromancer"
} | {
    type: "mafiaSupportWildcard"
    role: Role
} | {
    type: "jester"
} | {
    type: "revolutionary"
} | 
Doomsayer 
| {
    type: "politician"
} | {
    type: "witch"
} | {
    type: "scarecrow"
} | {
    type: "warper"
} | {
    type: "kidnapper"
    executionsRemaining: number,
    jailedTargetRef: number | null
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
} | {
    type: "ojo"
    roleChosen: Role | null,
    chosenOutline: [number | null, number | null],
    previouslyGivenResults: [number, AuditorResult][]
} | {
    type: "puppeteer"
    action: PuppeteerAction,
    marionettesRemaining: number
} | {
    type: "kira"
    guesses: Partial<Record<PlayerIndex, KiraGuess>>
} | {
    type: "fiendsWildcard"
    role: Role
} | {
    type: "apostle"
} | {
    type: "disciple"
} | {
    type: "zealot"
} | {
    type: "serialKiller"
}


export type Role = keyof typeof ROLES;
export type SingleRoleJsonData = {
    mainRoleSet: RoleSet,
    roleSets: RoleSet[],
    armor: boolean,
    aura: null | "innocent" | "suspicious",
    maxCount: null | number,
    roleSpecificMenu: boolean,
    canWriteDeathNote: boolean,
    canBeConvertedTo: Role[],
    chatMessages: ChatMessageVariant[] 
}
export type RoleJsonData = Record<Role, SingleRoleJsonData>

export function getMainRoleSetFromRole(role: Role): RoleSet {
    return roleJsonData()[role].mainRoleSet as RoleSet;
}

export function roleJsonData(): RoleJsonData {
    return ROLES as RoleJsonData;
}

export function getSingleRoleJsonData(role: Role): SingleRoleJsonData {
    return roleJsonData()[role];
}