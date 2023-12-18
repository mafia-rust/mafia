import translate from "./lang";
import { Role, getFactionAlignmentFromRole, getFactionFromRole } from "./roleState.d";
import ROLES from "./../resources/roles.json";

export const FACTIONS = ["town", "mafia", "neutral"] as const;
export type Faction = typeof FACTIONS[number]
export function getAllFactionAlignments(faction: Faction): FactionAlignment[] {
    switch(faction){
        case "town": return [
            "townPower", "townKilling", "townProtective", "townInvestigative", "townSupport"
        ];
        case "mafia": return [
            "mafiaKilling", "mafiaDeception", "mafiaSupport", "mafiaPower"
        ];
        case "neutral": return [
            "neutralKilling", "neutralEvil", "neutralChaos"
        ];
    }
}
export function getRoleOutlineFromFaction(faction: Faction): RoleOutline {
    return {
        type: "faction",
        faction: faction
    }
}

export const FACTION_ALIGNMENTS = [
    "townPower","townKilling","townProtective","townInvestigative","townSupport",
    "mafiaKilling","mafiaDeception","mafiaSupport","mafiaPower",
    "neutralKilling","neutralEvil","neutralChaos"
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
        case "mafiaPower": return "mafia";

        case "neutralKilling": return "neutral";
        case "neutralEvil": return "neutral";
        case "neutralChaos": return "neutral";
    }
}
export function getAlignmentStringFromFactionAlignment(factionAlignment: FactionAlignment): string {
    const alignment = factionAlignment.replace(getFactionFromFactionAlignment(factionAlignment).toString(), "");
    return alignment.charAt(0).toLowerCase() + alignment.slice(1);
}
export function getRoleOutlineFromFactionAlignment(factionAlignment: FactionAlignment): RoleOutline {
    return {
        type: "factionAlignment",
        factionAlignment: factionAlignment
    }
}


export type RoleOutline = ({
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
});
export type RoleOutlineType = RoleOutline["type"];

export function getFactionFromRoleOutline(roleOutline: RoleOutline): Faction | null {
    switch(roleOutline.type){
        case "any": return null;
        case "faction": return roleOutline.faction;
        case "factionAlignment": return getFactionFromFactionAlignment(roleOutline.factionAlignment);
        case "exact": return getFactionFromRole(roleOutline.role);
    }
}
export function getFactionAlignmentFromRoleOutline(roleOutline: RoleOutline): FactionAlignment | null {
    switch(roleOutline.type){
        case "any": return null;
        case "faction": return null;
        case "factionAlignment": return roleOutline.factionAlignment;
        case "exact": return getFactionAlignmentFromRole(roleOutline.role);
    }
}

export function translateRoleOutline(roleOutline: RoleOutline): string | null {
    if(roleOutline.type === "any"){
        return translate("any");
    }
    if(roleOutline.type === "faction"){
        return translate("faction."+roleOutline.faction.toString())+" "+translate("any");
    }
    if(roleOutline.type === "factionAlignment"){
        return translate("faction."+getFactionFromFactionAlignment(roleOutline.factionAlignment))+" "+translate("alignment."+getAlignmentStringFromFactionAlignment(roleOutline.factionAlignment));
    }
    if(roleOutline.type === "exact"){
        return translate("role."+roleOutline.role+".name");
    }
    return null
}

export function sortRoleOutlines(roleOutlines: RoleOutline[]): RoleOutline[] {
    //sorts by type, then by faction, then by factionAlignment, then by role
    //need to get faction and factionAlignment from role

    let factionOrder = ["town", "mafia", "neutral"];
    let factionAlignmentOrder = [
        "townPower","townInvestigative","townProtective","townKilling","townSupport",
        "mafiaKilling","mafiaSupport","mafiaDeception","mafiaPower",
        "neutralEvil","neutralKilling","neutralChaos"
    ];

    
    return roleOutlines.sort((a, b)=>{
        if(a.type === "any" && b.type === "any") return 0;
        if(a.type === "any") return 1;
        if(b.type === "any") return -1;

        if(a.type === "faction" && b.type === "faction"){
            //sort by faction
            let aFactionIndex = factionOrder.indexOf(a.faction);
            let bFactionIndex = factionOrder.indexOf(b.faction);
            if(aFactionIndex !== bFactionIndex) return aFactionIndex - bFactionIndex;
        }
        if(a.type === "faction") return 1;
        if(b.type === "faction") return -1;

        if(a.type === "factionAlignment" && b.type === "factionAlignment"){
            let aFactionAlignmentIndex = factionAlignmentOrder.indexOf(a.factionAlignment);
            let bFactionAlignmentIndex = factionAlignmentOrder.indexOf(b.factionAlignment);
            if(aFactionAlignmentIndex !== bFactionAlignmentIndex) return aFactionAlignmentIndex - bFactionAlignmentIndex;
        }
        if(a.type === "factionAlignment") return 1;
        if(b.type === "factionAlignment") return -1;

        if(a.type === "exact" && b.type === "exact"){
            //sort roles by order in ROLES
            let aRoleIndex = Object.keys(ROLES).indexOf(a.role);
            let bRoleIndex = Object.keys(ROLES).indexOf(b.role);
            if(aRoleIndex !== bRoleIndex) return aRoleIndex - bRoleIndex;
        }
        if(a.type === "exact") return 1;
        if(b.type === "exact") return -1;

        return 0;
    });
}