import { ListMapData } from "../ListMap";
import { KiraGuess } from "../menu/game/gameScreenContent/AbilityMenu/AbilitySelectionTypes/KiraSelectionMenu";
import { PhaseType, PlayerIndex } from "./gameState.d";
import translate, { translateChecked } from "./lang";
import { Role } from "./roleState.d";
import abilitiesJson from "../resources/abilityId.json";


export type AbilityJsonData = Partial<Record<ControllerIDLink, SingleAbilityJsonData>>;
export type SingleAbilityJsonData = {
    midnight: boolean,
}

export function allAbilitiesJsonData(): AbilityJsonData {
    return abilitiesJson;
}
export function singleAbilityJsonData(link: ControllerIDLink): SingleAbilityJsonData | null {
    return allAbilitiesJsonData()[link]??null;
}


export type AbilityInput = {
    id: ControllerID, 
    selection: AbilitySelection
}


export type SavedControllersMap = {
    save: ListMapData<ControllerID, SavedController>
}

export type SavedController = {
    selection: AbilitySelection,
    availableAbilityData: {
        available: AvailableAbilitySelection,
        grayedOut: boolean,
        resetOnPhaseStart: PhaseType | null,
        dontSave: boolean
        defaultSelection: AbilitySelection,
        allowedPlayers: PlayerIndex[]
    }
}

export type ControllerID = {
    type: "role",
    player: PlayerIndex,
    role: Role,
    id: RoleControllerID,
} | {
    type: "forfeitVote",
    player: PlayerIndex,
} | {
    type: "pitchforkVote",
    player: PlayerIndex,
} | {
    type: "syndicateGunItemShoot",
} | {
    type: "syndicateGunItemGive",
} | {
    type: "syndicateChooseBackup"
} | {
    type: "syndicateBackupAttack"
}

export type RoleControllerID = number;

/// create a type that represnts all strings that look like "abilityId/role/1"

export type ControllerIDLink = (
    `role/${Role}/${RoleControllerID}` | 
    `${ControllerID["type"]}`
);

export function controllerIdToLink(id: ControllerID): ControllerIDLink {
    let out: ControllerIDLink = `${id.type}`;
    if (id.type === "role") {
        out += `/${id.role}/${id.id}`;
    }
    return out as ControllerIDLink;
}

/// if it doesnt exist then returns ""
export function translateControllerID(
    abilityId: ControllerID
): string {
    switch (abilityId.type) {
        case "role":
            return translate("role."+abilityId.role+".name")+" "+(translateChecked("controllerId.role."+abilityId.role+"."+abilityId.id+".name")??"");
        default:
            return translateChecked("controllerId."+abilityId.type+".name")??"";
    }
}
export function translateControllerIDNoRole(
    abilityId: ControllerID
): string {
    switch (abilityId.type) {
        case "role":
            return (translateChecked("controllerId.role."+abilityId.role+"."+abilityId.id+".name")??"");
        default:
            return translateChecked("controllerId."+abilityId.type+".name")??"";
    }
}

export function sortControllerIdCompare(
    a: ControllerID, 
    b: ControllerID, 
    firstRole: Role | null = null
): number {
    // Define priority order for each type
    const typePriority: Record<string, number> = {
        role: 1,
        syndicateGunItemShoot: 2,
        syndicateGunItemGive: 3,
        syndicateChooseBackup: 4,
        syndicateBackupAttack: 5,
        forfeitVote: 6,
        pitchforkVote: 7
    };

    // Get priority of types
    const aPriority = typePriority[a.type] ?? Infinity;
    const bPriority = typePriority[b.type] ?? Infinity;

    // Compare types by priority
    if (aPriority !== bPriority) {
        return aPriority - bPriority;
    }

    // For "role", further sorting by player, role, and id
    if (a.type === "role" && b.type === "role") {
        // If firstRole is provided, prioritize it
        if (firstRole && a.role === firstRole && b.role !== firstRole) return -1;
        if (firstRole && b.role === firstRole && a.role !== firstRole) return 1;

        // Compare by player index
        if (a.player !== b.player) {
            return a.player - b.player;
        }

        // Compare by role name (or id if roles are the same)
        const roleComparison = a.role.localeCompare(b.role);
        if (roleComparison !== 0) {
            return roleComparison;
        }

        // Compare by id as a fallback
        return a.id - b.id;
    }

    // Default fallback: equal priority
    return 0;
}

export type AbilitySelection = {
    type: "unit",
} | {
    type: "boolean"
    selection: BooleanSelection
} | {
    type: "twoPlayerOption"
    selection: TwoPlayerOptionSelection
} | {
    type: "playerList",
    selection: PlayerListSelection
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
    type: "string",
    selection: StringSelection
} | {
    type: "integer",
    selection: IntegerSelection
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
        case "twoPlayerOption":
            return {type: "twoPlayerOption", selection: null};
        case "playerList":
            return {type: "playerList", selection: []};
        case "roleOption":
            return {type: "roleOption", selection: null};
        case "twoRoleOption":
            return {type: "twoRoleOption", selection: [null, null]};
        case "twoRoleOutlineOption":
            return {type: "twoRoleOutlineOption", selection: [null, null]};
        case "string":
            return {type: "string", selection: ""};
        case "integer":
            return {type: "integer", selection: 0};
        case "kira":
            return {type: "kira", selection: []};
    }
}


export type AvailableAbilitySelection = {
    type: "unit",
} | {
    type: "boolean",
} | {
    type: "twoPlayerOption"
    selection: AvailableTwoPlayerOptionSelection,
} | {
    type: "playerList",
    selection: AvailablePlayerListSelection,
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
    type: "string"
} | {
    type: "integer",
    selection: AvailableIntegerSelection
} | {
    type: "kira",
    selection: AvailableKiraSelection
}


export type BooleanSelection = boolean;

export type TwoPlayerOptionSelection = [number, number] | null;
export type AvailableTwoPlayerOptionSelection = {
    availableFirstPlayers: number[],
    availableSecondPlayers: number[],
    canChooseDuplicates: boolean,
    canChooseNone: boolean
}

export type PlayerListSelection = PlayerIndex[];
export type AvailablePlayerListSelection = {
    availablePlayers: PlayerIndex[],
    canChooseDuplicates: boolean,
    maxPlayers: number | null
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

export type StringSelection = string;

export type IntegerSelection = number;
export type AvailableIntegerSelection = {
    min: number,
    max: number
};

export type KiraSelection = ListMapData<PlayerIndex, KiraGuess>
export type AvailableKiraSelection = {
    countMustGuess: number
};