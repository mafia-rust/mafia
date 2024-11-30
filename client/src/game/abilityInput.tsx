import { ListMapData } from "../ListMap";
import { KiraGuess } from "../menu/game/gameScreenContent/AbilityMenu/AbilitySelectionTypes/KiraSelectionMenu";
import { PhaseType, PlayerIndex } from "./gameState.d";
import translate from "./lang";
import { Role } from "./roleState.d";


export type AbilityInput = {
    id: AbilityID, 
    selection: AbilitySelection
}


export type PlayerSavedAbilities = {
    save: ListMapData<AbilityID, SavedSingleAbility>
}

export type SavedSingleAbility = {
    selection: AbilitySelection,
    availableAbilityData: {
        available: AvailableAbilitySelection,
        grayed_out: boolean,
        reset_on_phase_start: PhaseType | null,
        default_selection: AbilitySelection
    }
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
} | {
    type: "kira",
    selection: KiraSelection
}

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
        case "kira":
            return {type: "kira", selection: []};
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
} | {
    type: "kira",
    selection: AvailableKiraSelection
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

export type TwoRoleOutlineOptionSelection = [PlayerIndex | null, PlayerIndex | null];
export type AvailableTwoRoleOutlineOptionSelection = (number | null)[];

export type KiraSelection = ListMapData<PlayerIndex, KiraGuess>
export type AvailableKiraSelection = {
    countMustGuess: number
};