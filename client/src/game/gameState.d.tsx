import { Grave } from "./grave";
import { ChatMessage } from "./chatMessage";
import ROLES from "./../resources/roles.json";

export default interface GameState {
    myName: string | null,
    myIndex: PlayerIndex | null,

    chatMessages : ChatMessage[],
    graves: Grave[],
    players: Player[],
    
    playerOnTrial: PlayerIndex | null,
    phase: Phase | null,
    secondsLeft: number,
    dayNumber: number,

    role: Role | null,
    roleData: RoleData | null,

    will: string,
    notes: string,
    targets: PlayerIndex[],
    voted: PlayerIndex | null,
    judgement: Verdict,
    
    roleList: RoleListEntry[],
    investigatorResults: Role[][],
    phaseTimes: PhaseTimes
}

export type PlayerIndex = number;
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

export interface Player {
    name: string,
    index: number
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    },
    numVoted: number,
    alive: boolean,
    roleLabel: Role | null,

    toString(): string
}

export type RoleData = {
    role: "jailor",
    executionsRemaining: number,
    jailedTargetRef: number | null
} | {
    role: "sheriff"
} | {
    role: "lookout"
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
    role:"veteran"
    alertsRemaining: number,
} | {
    role:"escort"
} | {
    role:"medium"
} | {
    role:"retributionist"
} | {
    role:"mafioso"
} | {
    role:"consort"
} | {
    role:"blackmailer"
} | {
    role:"janitor"
    cleansRemaining: number,
} | {
    role:"covenLeader"
} | {
    role:"voodooMaster"
}

export type Role = string;
export function getFactionFromRole(role: Role): Faction {
    return getFactionFromFactionAlignment(getFactionAlignmentFromRole(role));
}
export function getFactionAlignmentFromRole(role: Role): FactionAlignment {
    return ROLES[role as keyof typeof ROLES].factionAlignment as FactionAlignment;
}

export const FACTIONS = ["town", "mafia", "neutral", "coven"] as const;
export type Faction = typeof FACTIONS[number]
export function getAllFactionAlignments(faction: Faction): FactionAlignment[] {
    switch(faction){
        case "town": return [
            "townPower", "townKilling", "townProtective", "townInvestigative", "townSupport"
        ];
        case "mafia": return [
            "mafiaKilling", "mafiaDeception", "mafiaSupport"
        ];
        case "neutral": return [
            "neutralKilling", "neutralEvil", "neutralChaos"
        ];
        case "coven": return [
            "covenPower", "covenKilling", "covenUtility", "covenDeception"
        ];
    }
}

export const FACTION_ALIGNMENTS = [
    "townPower","townKilling","townProtective","townInvestigative","townSupport",
    "mafiaKilling","mafiaDeception","mafiaSupport",
    "neutralKilling","neutralEvil","neutralChaos",
    "covenPower","covenKilling","covenUtility","covenDeception"
] as const;
export type FactionAlignment = typeof FACTION_ALIGNMENTS[number]

export function getAllRolesFromFactionAlignment(factionAlignment: FactionAlignment): Role[] {
    return (ROLES as any).filter(
        (role: { factionAlignment: FactionAlignment; }) => role.factionAlignment === factionAlignment
    );
}
export function getFactionFromFactionAlignment(factionAlignment: FactionAlignment): Faction {
    switch(factionAlignment){
        case "townPower": return "town";
        case "townKilling": return "town";
        case "townProtective": return "town";
        case "townInvestigative": return "town";
        case "townSupport": return "town";

        case "mafiaKilling": return "mafia";
        case "mafiaDeception": return "mafia";
        case "mafiaSupport": return "mafia";

        case "neutralKilling": return "neutral";
        case "neutralEvil": return "neutral";
        case "neutralChaos": return "neutral";

        case "covenPower": return "coven";
        case "covenKilling": return "coven";
        case "covenUtility": return "coven";
        case "covenDeception": return "coven";
    }
}
export function getAlignmentStringFromFactionAlignment(factionAlignment: FactionAlignment): string {
    //make first letter lowercase
    let alignment = factionAlignment.replace(getFactionFromFactionAlignment(factionAlignment).toString(), "");
    return alignment.charAt(0).toLowerCase() + alignment.slice(1);
}


export type RoleListEntry={
    type: "any",
} | {
    type: "faction",
    faction: Faction,
} | {
    type: "factionAlignment",
    factionAlignment: FactionAlignment,
} | {
    type: "exact",
    role: Role,
};
export type RoleListEntryType = "any"|"faction"|"factionAlignment"|"exact";

export function getFactionFromRoleListEntry(roleListEntry: RoleListEntry): Faction | null {
    switch(roleListEntry.type){
        case "any": return null;
        case "faction": return roleListEntry.faction;
        case "factionAlignment": return getFactionFromFactionAlignment(roleListEntry.factionAlignment);
        case "exact": return getFactionFromRole(roleListEntry.role);
    }
}
export function getFactionAlignmentFromRoleListEntry(roleListEntry: RoleListEntry): FactionAlignment | null {
    switch(roleListEntry.type){
        case "any": return null;
        case "faction": return null;
        case "factionAlignment": return roleListEntry.factionAlignment;
        case "exact": return getFactionAlignmentFromRole(roleListEntry.role);
    }
}