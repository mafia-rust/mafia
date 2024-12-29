
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
    "town", "townCommon", "townInvestigative", "townProtective", "townKilling", "townSupport", 
    "mafia", "mafiaKilling", "mafiaSupport",
    "neutral", "minions",
    "fiends",
    "cult"
] as const;
export type RoleSet = typeof ROLE_SETS[number];
export function getRolesFromRoleSet(roleSet: RoleSet): Role[] {
    const ROLES = roleJsonData();
    return getAllRoles().filter((role) => {
        return ROLES[role].roleSets.includes(roleSet);
    });
}
export function getRoleSetsFromRole(role: Role): RoleSet[] {
    return ROLE_SETS.filter((roleSet) => {
        return getRolesFromRoleSet(roleSet).includes(role);
    })
}


export type RoleOutlineType = RoleOutline["type"];
export type RoleOutline = ({
    type: "any",
} | {
    type: "roleOutlineOptions",
    options: RoleOutlineOption[],
});


export type RoleOutlineOptionType = RoleOutlineOption["type"];
export type RoleOutlineOption = ({
    type: "roleSet",
    roleSet: RoleSet,
} | {
    type: "role",
    role: Role,
});




export function translateRoleOutline(roleOutline: RoleOutline): string {
    switch(roleOutline.type){
        case "any":
            return translate("any");
        case "roleOutlineOptions":
            return roleOutline.options.map(translateRoleOutlineOption).join(" "+translate("union")+" ");
    }
}
export function translateRoleOutlineOption(roleOutlineOption: RoleOutlineOption): string {
    switch(roleOutlineOption.type){
        case "roleSet":
            return translate(roleOutlineOption.roleSet);
        case "role":
            return translate("role."+roleOutlineOption.role+".name");
    }
}
export function getRolesFromOutline(roleOutline: RoleOutline): Role[] {
    switch(roleOutline.type){
        case "any":
            return getAllRoles();
        case "roleOutlineOptions":
            return roleOutline.options.flatMap((option) => getRolesFromOutlineOption(option));
    }
}
export function getRolesFromOutlineOption(roleOutlineOption: RoleOutlineOption): Role[] {
    switch(roleOutlineOption.type){
        case "roleSet":
            return getRolesFromRoleSet(roleOutlineOption.roleSet);
        case "role":
            return [roleOutlineOption.role];
    }
}

export function simplifyRoleOutline(roleOutline: RoleOutline): RoleOutline {

    if(roleOutline.type === "any") return roleOutline;

    let newOptions = [...roleOutline.options];

    for(let optionA of roleOutline.options){
        for(let optionB of roleOutline.options){
            if(outlineOptionIsSubset(optionA, optionB) && optionA !== optionB){
                newOptions = newOptions.filter((option) => option !== optionA);
            }
        }
    }

    newOptions = newOptions.sort(outlineOptionCompare);
    return {type: "roleOutlineOptions", options: newOptions};
    
    
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