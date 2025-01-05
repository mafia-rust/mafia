
import { Conclusion, InsiderGroup, translateWinCondition } from "./gameState.d";
import translate from "./lang";
import { Role, roleJsonData } from "./roleState.d";

export type RoleList = RoleOutline[];
export function getRolesFromRoleList(roleList: RoleList): Role[] {

    let set = new Set<Role>();
    for(let roleOutline of roleList){
        for(let role of getRolesFromOutline(roleOutline)){
            set.add(role);
        }
    }

    return Array.from(set);
}

export function getRolesComplement(roleList: Role[]): Role[] {
    return getAllRoles().filter((role) => {
        return !roleList.includes(role);
    });
}



export const ROLE_SETS = [
    "any",
    "town", "townCommon", "townInvestigative", "townProtective", "townKilling", "townSupport", 
    "mafia", "mafiaKilling", "mafiaSupport",
    "neutral", "minions",
    "fiends",
    "cult"
] as const;
export type RoleSet = typeof ROLE_SETS[number];
export function getRolesFromRoleSet(roleSet: RoleSet): Role[] {
    return getAllRoles().filter((role) => {
        return getRoleSetsFromRole(role).includes(roleSet);
    });
}
export function getRoleSetsFromRole(role: Role): RoleSet[] {
    const ROLES = roleJsonData();
    return [...ROLES[role].roleSets, "any"]
}


export type RoleOutline = RoleOutlineOption[];

export type RoleOutlineOption = ({
    roleSet: RoleSet
    role?: undefined
} | {
    roleSet?: undefined
    role: Role
}) & {
    winIfAny?: Conclusion[],
    insiderGroups?: InsiderGroup[]
}

export type RoleOrRoleSet = ({
    type: "roleSet",
    roleSet: RoleSet
} | {
    type: "role",
    role: Role
})




export function translateRoleOutline(roleOutline: RoleOutline): string {
    return roleOutline.map(translateRoleOutlineOption).join(" "+translate("union")+" ")
}
export function translateRoleOutlineOption(roleOutlineOption: RoleOutlineOption): string {
    let out = "";
    if (roleOutlineOption.insiderGroups) {
        if (roleOutlineOption.insiderGroups.length === 0) {
            out += translate("chatGroup.all.icon")
        }
        for (const insiderGroup of roleOutlineOption.insiderGroups) {
            out += translate(`chatGroup.${insiderGroup}.icon`) + ' '
        }
    }
    if (roleOutlineOption.winIfAny) {
        out += `${translateWinCondition({ type: "gameConclusionReached", winIfAny: roleOutlineOption.winIfAny })} `
    }
    if (roleOutlineOption.roleSet) {
        out += translate(roleOutlineOption.roleSet)
    } else if (roleOutlineOption.role) {
        out += translate("role."+roleOutlineOption.role+".name")
    }
    return out;
}
export function translateRoleOrRoleSet(roleOrRoleSet: RoleOrRoleSet): string {
    switch (roleOrRoleSet.type) {
        case "roleSet":
            return translate(roleOrRoleSet.roleSet)
        case "role":
            return translate("role."+roleOrRoleSet.role+".name")
    }
}
export function getRolesFromOutline(roleOutline: RoleOutline): Role[] {
    return roleOutline.flatMap((option) => getRolesFromOutlineOption(option));
}
export function getRolesFromOutlineOption(roleOutlineOption: RoleOutlineOption): Role[] {
    if (roleOutlineOption.roleSet) {
        return getRolesFromRoleSet(roleOutlineOption.roleSet)
    } else {
        return [roleOutlineOption.role]
    }
}
export function getRolesFromRoleOrRoleSet(roleOrRoleSet: RoleOrRoleSet): Role[] {
    switch (roleOrRoleSet.type) {
        case "roleSet":
            return getRolesFromRoleSet(roleOrRoleSet.roleSet)
        case "role":
            return [roleOrRoleSet.role]
    }
}

export function simplifyRoleOutline(roleOutline: RoleOutline): RoleOutline {
    let newOptions = [...roleOutline];

    for(let optionA of roleOutline){
        for(let optionB of roleOutline){
            if(outlineOptionIsSubset(optionA, optionB) && optionA !== optionB){
                newOptions = newOptions.filter((option) => option !== optionA);
            }
        }
    }

    newOptions = newOptions.sort(outlineOptionCompare);
    return newOptions;
}
function outlineOptionIsSubset(optionA: RoleOutlineOption, optionB: RoleOutlineOption): boolean {
    let rolesA = getRolesFromOutlineOption(optionA);
    let rolesB = getRolesFromOutlineOption(optionB);
    return rolesA.every((role) => rolesB.includes(role));
}
function outlineOptionCompare(optionA: RoleOutlineOption, optionB: RoleOutlineOption): number {
    let rolesA = getRolesFromOutlineOption(optionA);
    let rolesB = getRolesFromOutlineOption(optionB);
    return rolesB.length - rolesA.length;
}

export function getAllRoles(): Role[] {
    return Object.entries(roleJsonData())
        .sort((a, b) => translate(`role.${a[0]}.name`).localeCompare(translate(`role.${b[0]}.name`)))
        .sort((a, b) => ROLE_SETS.indexOf(a[1].mainRoleSet) - ROLE_SETS.indexOf(b[1].mainRoleSet))
        .map((a) => a[0]) as Role[];
}