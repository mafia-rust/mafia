import { Role } from "../../game/roleState.d";
import { PHASES, PhaseTimes, PhaseType } from "../../game/gameState.d";
import { GameMode, GameModeData, SavedGameModes } from "../../game/localStorage";
import { FACTIONS, Faction, ROLE_SETS, RoleList, RoleOutline, RoleOutlineOption, RoleSet, getAllRoles } from "../../game/roleListState.d";

export type ParseSuccess<T> = {
    type: "success",
    value: T
}
export type ParseFailure = {
    type: "failure",
    reason: "gameModeNotObject" | "invalidJSON" | `${keyof GameMode}KeyMissingFromGameMode` | "nameIsNotString" | "roleListIsNotArray" | "roleOutlineMissingTypeKey" | 
        "roleOutlineMissingOptionsKey" | "roleOutlineInvalidType" | "roleOutlineOptionListIsNotArray" | "roleOutlineOptionMissingTypeKey" | 
        "roleOutlineOptionMissingRoleKey" | "roleOutlineOptionMissingRoleSetKey" | "roleOutlineOptionMissingFactionKey" | "nameContainsNewline" |
        "roleOutlineOptionInvalidType" | `${PhaseType}KeyMissingFromPhaseTimes` | `${PhaseType}ValueOfPhaseTimesIsNotNumber` | "disabledRolesIsNotArray" | 
        "roleIsNotString" | "invalidRole" | "roleSetIsNotString" | "invalidRoleSet" | "factionIsNotString" | "invalidFaction" | "roleListIsEmpty" | 
        `${'name' | keyof GameModeData}KeyMissingFromGameModeData` | "gameModeDataRecordNotObject" | "gameModeDataRecordKeyNotNumber" | "gameModeStorageIsNotArray" |
        "gameModeDataRecordKeyDoesNotMatchRoleListLength",
    snippet: string
}
type ParseResult<T> = ParseSuccess<T> | ParseFailure;

export function isFailure(result: ParseResult<any>): result is ParseFailure {
    return result.type === "failure";
}

function Success<T>(result: T): ParseSuccess<T> {
    return {
        type: "success",
        value: result
    }
}

function Failure(reason: ParseFailure["reason"], snippet: any): ParseFailure {
    return {
        type: 'failure',
        reason,
        snippet: JSON.stringify(snippet)
    }
}

export function parseJsonObject(jsonString: string): NonNullable<any> | null {
    let json: any;
    try {
        console.log(`Parsing: ${jsonString}`);
        json = JSON.parse(jsonString);
    } catch {
        return null;
    }

    return json;
}

export function parseGameModeStorage(json: NonNullable<any>): ParseResult<SavedGameModes> {
    if (!Array.isArray(json)) {
        return Failure("gameModeStorageIsNotArray", json);
    }

    const gameModeList = json.map(parseGameMode);
    for (const gameMode of gameModeList) {
        if (isFailure(gameMode)) return gameMode;
    }

    return Success(gameModeList.map(success => (success as ParseSuccess<GameMode>).value) as GameMode[]);
}

export function parseGameMode(json: NonNullable<any>): ParseResult<GameMode> {
    for (const key of ['name', 'data']) {
        if (!Object.keys(json).includes(key)) {
            return Failure(`${key as keyof GameMode}KeyMissingFromGameMode`, json)
        }
    }

    const name = parseName(json.name);
    if (isFailure(name)) return name;

    const gameModeDataRecord = parseGameModeDataRecord(json.data);
    if (isFailure(gameModeDataRecord)) return gameModeDataRecord;

    return Success({
        name: name.value,
        data: gameModeDataRecord.value
    })
}

export default function parseShareableGameModeData(json: NonNullable<any>): ParseResult<GameModeData & { name?: string }> {
    const gameMode = parseGameModeData(json);
    if (isFailure(gameMode)) {
        return gameMode;
    } else {
        if (!Object.keys(json).includes('name')) {
            return gameMode;
        }

        const name = parseName(json.name);
        if (isFailure(name)) return name;

        return Success({ name: name.value, ...gameMode.value });
    }
}

export function parseGameModeDataRecord(json: NonNullable<any>): ParseResult<Record<number, GameModeData>> {
    if (typeof json !== "object" || Array.isArray(json)) {
        return Failure("gameModeDataRecordNotObject", json);
    }
    
    const parsedEntries: Record<number, GameModeData> = {};
    for (const [key, value] of Object.entries(json)) {
        let players;
        try {
            players = parseInt(key)
        } catch {
            return Failure("gameModeDataRecordKeyNotNumber", key);
        }

        const datum = parseGameModeData(value);

        if (isFailure(datum)) {
            return datum;
        }

        if (datum.value.roleList.length !== players) {
            return Failure("gameModeDataRecordKeyDoesNotMatchRoleListLength", json);
        }
        
        parsedEntries[players] = datum.value
    }

    return Success(parsedEntries);
}

export function parseGameModeData(json: NonNullable<any>): ParseResult<GameModeData> {
    if (typeof json !== "object" || Array.isArray(json)) {
        return Failure("gameModeNotObject", json);
    }

    for (const key of ['roleList', 'phaseTimes', 'disabledRoles']) {
        if (!Object.keys(json).includes(key)) {
            return Failure(`${key as keyof GameModeData}KeyMissingFromGameModeData`, json)
        }
    }

    const roleList = parseRoleList(json.roleList);
    if (isFailure(roleList)) return roleList;

    const phaseTimes = parsePhaseTimes(json.phaseTimes);
    if (isFailure(phaseTimes)) return phaseTimes;

    const disabledRoles = parseDisabledRoles(json.disabledRoles);
    if (isFailure(disabledRoles)) return disabledRoles;

    return Success({
        roleList: roleList.value, 
        phaseTimes: phaseTimes.value, 
        disabledRoles: disabledRoles.value 
    });
}

function parseName(json: NonNullable<any>): ParseResult<string> {
    if (typeof json !== "string") {
        return Failure("nameIsNotString", json)
    } else {
        if (json.includes('\n') || json.includes('\r')) {
            return Failure("nameContainsNewline", json)
        }
        return Success(json);
    }
}

function parseRoleList(json: NonNullable<any>): ParseResult<RoleList> {
    if (!Array.isArray(json)) {
        return Failure("roleListIsNotArray", json);
    }

    if (json.length === 0) {
        return Failure("roleListIsEmpty", json);
    }

    const roleList = json.map(parseRoleOutline);

    for (const outline of roleList) {
        if (isFailure(outline)) return outline;
    }

    return Success(roleList.map(success => (success as ParseSuccess<RoleOutline>).value));
}

function parseRoleOutline(json: NonNullable<any>): ParseResult<RoleOutline> {
    if (!Object.keys(json).includes('type')) {
        return Failure("roleOutlineMissingTypeKey", json);
    }

    switch (json.type) {
        case "any":
            return Success({ type: "any" })
        case "roleOutlineOptions":
            if (!Object.keys(json).includes('options')) {
                return Failure("roleOutlineMissingOptionsKey", json);
            }

            const options = parseRoleOutlineOptionList(json.options);
            if (isFailure(options)) return options;

            return Success({
                type: "roleOutlineOptions",
                options: options.value
            });
        default:
            return Failure("roleOutlineInvalidType", json);
    }
}

function parseRoleOutlineOptionList(json: NonNullable<any>): ParseResult<RoleOutlineOption[]> {
    if (!Array.isArray(json)) {
        return Failure("roleOutlineOptionListIsNotArray", json);
    }

    const outlineOptionList = json.map(parseRoleOutlineOption);
    for (const option of outlineOptionList) {
        if (isFailure(option)) return option;
    }

    return Success(outlineOptionList.map(success => (success as ParseSuccess<RoleOutlineOption>).value) as RoleOutlineOption[]);
}

function parseRoleOutlineOption(json: NonNullable<any>): ParseResult<RoleOutlineOption> {
    if (!Object.keys(json).includes('type')) {
        return Failure("roleOutlineOptionMissingTypeKey", json);
    }

    switch (json.type) {
        case "role":
            if (!Object.keys(json).includes('role')) {
                return Failure("roleOutlineOptionMissingRoleKey", json);
            }

            const role = parseRole(json.role);
            if (isFailure(role)) return role;

            return Success({
                type: "role",
                role: role.value
            });
        case "roleSet":
            if (!Object.keys(json).includes('roleSet')) {
                return Failure("roleOutlineOptionMissingRoleSetKey", json);
            }

            const roleSet = parseRoleSet(json.roleSet);
            if (isFailure(roleSet)) return roleSet;

            return Success ({
                type: "roleSet",
                roleSet: roleSet.value
            });
        case "faction":
            if (!Object.keys(json).includes('faction')) {
                return Failure("roleOutlineOptionMissingFactionKey", json);
            }

            const faction = parseFaction(json.faction);
            if (isFailure(faction)) return faction;

            return Success({
                type: "faction",
                faction: faction.value
            });
        default:
            return Failure("roleOutlineOptionInvalidType", json);
    }
}

function parsePhaseTimes(json: NonNullable<any>): ParseResult<PhaseTimes> {
    for (const phase of PHASES) {
        if (!Object.keys(json).includes(phase)) {
            return Failure(`${phase}KeyMissingFromPhaseTimes`, json);
        }
    }

    const phaseTimes = PHASES.reduce(
        (acc, phase) => {
            if (isFailure(acc)) return acc;

            try {
                return Success({
                    ...acc.value,
                    [phase]: Number(json[phase])
                })
            } catch {
                return Failure(`${phase}ValueOfPhaseTimesIsNotNumber`, json[phase]);
            }
        }, 
        Success({}) as ParseResult<Partial<PhaseTimes>>
    )

    return phaseTimes as ParseResult<PhaseTimes>;
}

function parseDisabledRoles(json: NonNullable<any>): ParseResult<Role[]> {
    if (!Array.isArray(json)) {
        return Failure("disabledRolesIsNotArray", json);
    }

    const listOfRoles = json.map(parseRole);
    for (const role of listOfRoles) {
        if (isFailure(role)) return role;
    }

    return Success(listOfRoles.map(role => (role as ParseSuccess<Role>).value) as Role[]);
}

function parseRole(json: NonNullable<any>): ParseResult<Role> {
    if (typeof json !== "string") {
        return Failure("roleIsNotString", json);
    }
    if (!getAllRoles().includes(json as Role)) {
        return Failure("invalidRole", json);
    }
    return Success(json as Role);
}

function parseRoleSet(json: NonNullable<any>): ParseResult<RoleSet> {
    if (typeof json !== "string") {
        return Failure("roleSetIsNotString", json);
    }
    if (!ROLE_SETS.includes(json as RoleSet)) {
        return Failure("invalidRoleSet", json);
    }
    return Success(json as RoleSet);
}

function parseFaction(json: NonNullable<any>): ParseResult<Faction> {
    if (typeof json !== "string") {
        return Failure("factionIsNotString", json);
    }
    if (!FACTIONS.includes(json as Faction)) {
        return Failure("invalidFaction", json)
    }
    return Success(json as Faction);
}