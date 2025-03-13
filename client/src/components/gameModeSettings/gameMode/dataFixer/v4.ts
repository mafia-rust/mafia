import { VersionConverter } from ".";
import { GameMode, GameModeData, GameModeStorage, ShareableGameMode } from "..";
import { Conclusion, CONCLUSIONS, INSIDER_GROUPS, InsiderGroup } from "../../../../game/gameState.d";
import { getDefaultSettings, Settings } from "../../../../game/localStorage";
import { RoleOutline, RoleOutlineOption, RoleSet } from "../../../../game/roleListState.d";
import { Role } from "../../../../game/roleState.d";
import { Failure, ParseResult, ParseSuccess, Success, isFailure } from "../parse";
import { parseName, parsePhaseTimes, parseRole, parseRoleSet } from "./initial";
import { parseEnabledModifiers, parseEnabledRoles } from "./v2";

const v4: VersionConverter = {
    convertSettings: parseSettings,

    convertShareableGameMode: parseShareableGameModeData,
    convertGameModeStorage: parseGameModeStorage
}

export default v4;

function parseSettings(json: NonNullable<any>): ParseResult<Settings> {
    if (typeof json !== "object" || Array.isArray(json)) {
        return Failure("settingsNotObject", json);
    }

    for(const key of ['format', 'volume', 'fontSize', 'accessibilityFont', 'defaultName', 'language', 'roleSpecificMenus']) {
        if (!Object.keys(json).includes(key)) {
            return Failure(`${key as keyof Settings}KeyMissingFromSettings`, json);
        }
    }

    for(const key of ['maxMenus', 'menuOrder']) {
        if (!Object.keys(json).includes(key)) {
            json.maxMenus = getDefaultSettings().maxMenus
            json.menuOrder = getDefaultSettings().menuOrder
        }
    }

    if (json.format !== "v4") {
        return Failure("settingsFormatNotV4", json);
    }
    
    const roleSpecificMenus = parseRoleSpecificMenus(json.roleSpecificMenus);
    if (isFailure(roleSpecificMenus)) return roleSpecificMenus;

    return Success(json);
}

function parseRoleSpecificMenus(json: NonNullable<any>): ParseResult<Role[]> {
    if (!Array.isArray(json)) {
        return Failure("roleSpecificMenusNotArray", json);
    }

    const roleList = json.map(parseRole);
    for (const role of roleList) {
        if (isFailure(role)) return role;
    }

    return Success(roleList.map(success => (success as ParseSuccess<Role>).value));
}

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
        format: "v4",
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

        return Success({ format: "v4", name: name.value, ...gameMode.value });
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
    const options = parseRoleOutlineOptionList(json);
    if (isFailure(options)) return options;

    return Success(options.value);
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

    let out: {
        insiderGroups?: InsiderGroup[],
        winIfAny?: Conclusion[],
        role?: Role,
        roleSet?: RoleSet
    } = {}


    if("insiderGroups" in json) {
        const insiderGroupsResult = parseRoleOutlineOptionInsiderGroups(json.insiderGroups);
        if (isFailure(insiderGroupsResult)) return insiderGroupsResult;
        out.insiderGroups = insiderGroupsResult.value;
    }

    if("winIfAny" in json) {
        const winIfAnyResult = parseRoleOutlineOptionWinIfAny(json.winIfAny);
        if (isFailure(winIfAnyResult)) return winIfAnyResult;
        out.winIfAny = winIfAnyResult.value;
    }

    if("role" in json && "roleSet" in json) {
        return Failure("roleOutlineOptionBothRoleAndRoleSet", json);
    }
    
    if ("role" in json) {
        const roleResult = parseRole(json.role);
        if (isFailure(roleResult)) return roleResult;
        out.role = roleResult.value;
    } else if ("roleSet" in json) {
        const roleSetResult = parseRoleSet(json.roleSet);
        if (isFailure(roleSetResult)) return roleSetResult;
        out.roleSet = roleSetResult.value;
    } else {
        return Failure("roleOutlineOptionNeitherRoleNorRoleSet", json);
    }

    return Success(out as RoleOutlineOption);
}


function parseRoleOutlineOptionWinIfAny(json: NonNullable<any>): ParseResult<Conclusion[]> {
    if (!Array.isArray(json)) {
        return Failure("winIfAnyNotArray", json);
    }
    
    const conclusions = json.map(parseConclusion);
    for (const conclusion of conclusions) {
        if (isFailure(conclusion)) return conclusion;
    }

    return Success(conclusions.map(success => (success as ParseSuccess<Conclusion>).value));
}

function parseConclusion(json: NonNullable<any>): ParseResult<Conclusion> {
    if (typeof json !== "string") {
        return Failure("conclusionNotString", json);
    }

    if (!CONCLUSIONS.includes(json as Conclusion)) {
        return Failure("conclusionInvalid", json);
    }

    return Success(json as Conclusion);
}


function parseRoleOutlineOptionInsiderGroups(json: NonNullable<any>): ParseResult<InsiderGroup[]> {
    if (!Array.isArray(json)) {
        return Failure("insiderGroupsNotArray", json);
    }

    const insiderGroups = json.map(parseInsiderGroup);
    for (const group of insiderGroups) {
        if (isFailure(group)) return group;
    }

    return Success(insiderGroups.map(success => (success as ParseSuccess<InsiderGroup>).value));
}

function parseInsiderGroup(json: NonNullable<any>): ParseResult<InsiderGroup> {
    if (typeof json !== "string") {
        return Failure("insiderGroupNotString", json);
    }

    if (!INSIDER_GROUPS.includes(json as InsiderGroup)) {
        return Failure("insiderGroupInvalid", json);
    }

    return Success(json as InsiderGroup);
}