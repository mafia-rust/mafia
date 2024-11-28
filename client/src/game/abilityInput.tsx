import { ReactElement } from "react";
import translate from "./lang";
import { Role } from "./roleState.d";


export type AbilityInput = {
    id: AbilityID, 
    selection: AbilitySelection
}


export type AbilityID = {
    type: "role",
    role: Role,
    id: RoleAbilityID,
} | {
    type: "forfeitVote",
} | {
    type: "pitchforkVote",
} | {
    type: "syndicateGunItemShoot",
} | {
    type: "syndicateGunItemGive",
};

export type RoleAbilityID = number;

export function abilityIdToString(id: AbilityID): string{
    let out = "abilityId/";
    out += id.type+"/";
    switch(id.type){
        case "role":
            out += id.id;
    }
    return out;
}

export function translateAbilityId(
    abilityId: AbilityID
): string {
    switch (abilityId.type) {
        case "role":
            return translate("ability.abilityId."+abilityId.role+"."+abilityId.id+".name");
        default:
            return translate("ability.abilityId."+abilityId.type+".name");
    }
}

export type AbilitySelection = {
    type: "unit",
} | {
    type: "boolean"
    selection: BooleanSelection
} | {
    type: "onePlayerOption"
    selection: OnePlayerOptionSelection
} | {
    type: "twoPlayerOption"
    selection: TwoPlayerOptionSelection
} | {
    type: "roleOption"
    selection: RoleOptionSelection
} | {
    type: "twoRoleOption"
    selection: TwoRoleOptionSelection
} | {
    type: "twoRoleOutlineOption"
    selection: TwoRoleOutlineOptionSelection
};

export function defaultAbilitySelection(available: AvailableAbilitySelection): AbilitySelection {
    switch (available.type) {
        case "unit":
            return {type: "unit"};
        case "boolean":
            return {type: "boolean", selection: false};
        case "onePlayerOption":
            return {type: "onePlayerOption", selection: null};
        case "twoPlayerOption":
            return {type: "twoPlayerOption", selection: [null, null]};
        case "roleOption":
            return {type: "roleOption", selection: null};
        case "twoRoleOption":
            return {type: "twoRoleOption", selection: [null, null]};
        case "twoRoleOutlineOption":
            return {type: "twoRoleOutlineOption", selection: [null, null]};
    }
}


export type AvailableAbilitySelection = {
    type: "unit",
} | {
    type: "boolean",
} | {
    type: "onePlayerOption"
    selection: AvailableOnePlayerOptionSelection,
} | {
    type: "twoPlayerOption"
    selection: AvailableTwoPlayerOptionSelection,
} | {
    type: "roleOption"
    selection: AvailableRoleOptionSelection,
} | {
    type: "twoRoleOption"
    selection: AvailableTwoRoleOptionSelection,
} | {
    type: "twoRoleOutlineOption"
    selection: AvailableTwoRoleOutlineOptionSelection,
}


export type BooleanSelection = boolean;

export type OnePlayerOptionSelection = number | null;
export type AvailableOnePlayerOptionSelection = (number | null)[];

export type TwoPlayerOptionSelection = [number | null, number | null];
export type AvailableTwoPlayerOptionSelection = {
    availableFirstPlayers: (number | null)[],
    availableSecondPlayers: (number | null)[],
    canChooseDuplicates: boolean
}

export type RoleOptionSelection = Role | null;
export type AvailableRoleOptionSelection = (Role | null)[];


export type TwoRoleOptionSelection = [Role | null, Role | null];
export type AvailableTwoRoleOptionSelection = {
    availableRoles: (Role | null)[],
    canChooseDuplicates: boolean
};

export type TwoRoleOutlineOptionSelection = [number | null, number | null];
export type AvailableTwoRoleOutlineOptionSelection = (number | null)[];