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
            return translate("role."+abilityId.role+".name")+" "+(translateChecked("controllerId."+abilityId.role+"."+abilityId.id+".name")??"");
        default:
            return translateChecked("controllerId."+abilityId.type+".name")??"";
    }
}
export function translateControllerIDNoRole(
    abilityId: ControllerID
): string {
    switch (abilityId.type) {
        case "role":
            return (translateChecked("controllerId."+abilityId.role+"."+abilityId.id+".name")??"");
        default:
            return translateChecked("controllerId."+abilityId.type+".name")??"";
    }
}

export function sortControllerIdCompare(a: ControllerID, b: ControllerID, firstRole: Role | null = null): number {
    if (a.type === "role" && b.type === "role") {
        if (a.role !== b.role && firstRole !== null) {
            if (a.role === firstRole) {
                return -1;
            }
            if (b.role === firstRole) {
                return 1;
            }
        }
        return a.id - b.id;
    }
    if (a.type === "role") {
        return -1;
    }
    if (b.type === "role") {
        return 1;
    }
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

export type KiraSelection = ListMapData<PlayerIndex, KiraGuess>
export type AvailableKiraSelection = {
    countMustGuess: number
};