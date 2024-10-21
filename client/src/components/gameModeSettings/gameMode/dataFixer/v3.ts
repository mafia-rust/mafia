import { VersionConverter } from ".";
import { GameMode, GameModeData, GameModeStorage, ShareableGameMode } from "..";
import { MODIFIERS, ModifierType } from "../../../../game/gameState.d";
import { RoleOutline, RoleOutlineOption } from "../../../../game/roleListState.d";
import { Role } from "../../../../game/roleState.d";
import { Failure, ParseResult, ParseSuccess, Success, isFailure } from "../parse";
import { parseName, parsePhaseTimes, parseRole, parseRoleSet } from "./initial";

const v3: VersionConverter = {
    convertShareableGameMode: parseShareableGameModeData,
    convertGameModeStorage: parseGameModeStorage
}

export default v3;


function parseGameModeStorage(json: NonNullable<any>): ParseResult<GameModeStorage> {
    if (typeof json !== "object" || Array.isArray(json)) {
        return Failure("gameModeStorageNotObject", json);
    }

    for (const key of ['format', 'gameModes']) {
        if (!Object.keys(json).includes(key)) {
            return Failure(`${key as keyof GameModeStorage}KeyMissingFromGameModeStorage`, json)
        }
    }

    const gameModeList = (json.gameModes as GameMode[]).map(parseGameMode);
    for (const gameMode of gameModeList) {
        if (isFailure(gameMode)) return gameMode;
    }

    return Success({
        format: "v3",
        gameModes: gameModeList.map(gameMode => (gameMode as ParseSuccess<GameMode>).value)
    })
}

function parseGameMode(json: NonNullable<any>): ParseResult<GameMode> {
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

function parseShareableGameModeData(json: NonNullable<any>): ParseResult<ShareableGameMode> {
    const gameMode = parseGameModeData(json);
    if (isFailure(gameMode)) {
        return gameMode;
    } else {
        if (!Object.keys(json).includes('name')) {
            return Failure("gameModeMissingNameKey", json);
        }

        const name = parseName(json.name);
        if (isFailure(name)) return name;

        return Success({ format: "v3", name: name.value, ...gameMode.value });
    }
}

function parseGameModeDataRecord(json: NonNullable<any>): ParseResult<Record<number, GameModeData>> {
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

function parseGameModeData(json: NonNullable<any>): ParseResult<GameModeData> {
    if (typeof json !== "object" || Array.isArray(json)) {
        return Failure("gameModeDataNotObject", json);
    }

    for (const key of ['roleList', 'phaseTimes', 'enabledRoles', 'enabledModifiers']) {
        if (!Object.keys(json).includes(key)) {
            return Failure(`${key as keyof GameModeData}KeyMissingFromGameModeData`, json)
        }
    }

    const roleList = parseRoleList(json.roleList);
    if (isFailure(roleList)) return roleList;

    const phaseTimes = parsePhaseTimes(json.phaseTimes);
    if (isFailure(phaseTimes)) return phaseTimes;

    const enabledRoles = parseEnabledRoles(json.enabledRoles);
    if (isFailure(enabledRoles)) return enabledRoles;

    const enabledModifiers = parseEnabledModifiers(json.enabledModifiers);
    if (isFailure(enabledModifiers)) return enabledModifiers;

    return Success({
        roleList: roleList.value, 
        phaseTimes: phaseTimes.value, 
        enabledRoles: enabledRoles.value,
        enabledModifiers: enabledModifiers.value
    });
}

function parseRoleList(json: NonNullable<any>): ParseResult<RoleOutline[]> {
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
        default:
            return Failure("roleOutlineOptionInvalidType", json);
    }
}


export function parseEnabledRoles(json: NonNullable<any>): ParseResult<Role[]> {
    if (!Array.isArray(json)) {
        return Failure("enabledRolesIsNotArray", json);
    }

    const listOfRoles = json.map(parseRole);
    for (const role of listOfRoles) {
        if (isFailure(role)) return role;
    }

    return Success(listOfRoles.map(role => (role as ParseSuccess<Role>).value) as Role[]);
}

export function parseEnabledModifiers(json: NonNullable<any>): ParseResult<ModifierType[]> {
    if (!Array.isArray(json)) {
        return Failure("enabledModifiersIsNotArray", json);
    }

    const listOfModifiers = json.map(parseModifier);
    for (const modifier of listOfModifiers) {
        if (isFailure(modifier)) return modifier;
    }

    return Success(listOfModifiers.map(modifier => (modifier as ParseSuccess<ModifierType>).value) as ModifierType[]);
}

export function parseModifier(json: NonNullable<any>): ParseResult<ModifierType> {

    if (typeof json !== "string") {
        return Failure("modifierIsNotString", json);
    }
    if (!Object.values(MODIFIERS).includes(json as ModifierType)) {
        return Failure("invalidModifier", json);
    }
    return Success(json as ModifierType);
}