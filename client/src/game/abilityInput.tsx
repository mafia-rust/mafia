import { Role } from "./roleState.d";


export type AbilityInput = {
    id: AbilityID, 
    selection: AbilitySelection
}


export type AbilityID = {
    type: "role"
    roleAbilityId: RoleAbilityID
} | {
    type: "forfeitVote",
} | {
    type: "pitchforkVote",
} | {
    type: "syndicateGunItemShoot",
} | {
    type: "syndicateGunItemGive",
}

export type RoleAbilityID = number;

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


export type AvailableAbilitySelection = {
    type: "unit",
} | {
    type: "booleanSelection",
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


export type BooleanSelection = boolean | null;

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