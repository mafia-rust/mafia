import translate, { styleText } from "./lang";
import { Role, getFactionAlignmentFromRole, getFactionFromRole } from "./roleState.d";


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
export function getRoleListEntryFromFaction(faction: Faction): RoleListEntry {
    return {
        type: "faction",
        faction: faction
    }
}

export const FACTION_ALIGNMENTS = [
    "townPower","townKilling","townProtective","townInvestigative","townSupport",
    "mafiaKilling","mafiaDeception","mafiaSupport",
    "neutralKilling","neutralEvil","neutralChaos",
    "covenPower","covenKilling","covenUtility","covenDeception"
] as const;
export type FactionAlignment = typeof FACTION_ALIGNMENTS[number]

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
export function getRoleListEntryFromFactionAlignment(factionAlignment: FactionAlignment): RoleListEntry {
    return {
        type: "factionAlignment",
        factionAlignment: factionAlignment
    }
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
export type RoleListEntryType = RoleListEntry["type"];

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

export function  renderRoleListEntry(roleListEntry: RoleListEntry): JSX.Element[] | null{
    if(roleListEntry.type === "any"){
        return styleText(translate("any"))
    }
    if(roleListEntry.type === "faction"){
        return styleText(translate("faction."+roleListEntry.faction.toString())+" "+translate("any"))
    }
    if(roleListEntry.type === "factionAlignment"){
        return styleText(translate("faction."+getFactionFromFactionAlignment(roleListEntry.factionAlignment))+" "+translate("alignment."+getAlignmentStringFromFactionAlignment(roleListEntry.factionAlignment)))
    }
    if(roleListEntry.type === "exact"){
        return styleText(translate("role."+roleListEntry.role+".name"))
    }
    return null
}