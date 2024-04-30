import { Settings } from "../menu/Settings";
import { PhaseTimes } from "./gameState.d";
import { RoleList } from "./roleListState.d";
import { Role } from "./roleState.d";
import DEFAULT_GAME_MODES from "../resources/defaultGameModes.json";

export function saveReconnectData(roomCode: number, playerId: number) {
    localStorage.setItem(
        "reconnectData",
        JSON.stringify({
            "roomCode": roomCode,
            "playerId": playerId,
            "lastSaveTime": Date.now()
        })
    );
}
export function deleteReconnectData() {
    localStorage.removeItem("reconnectData");
}
export function loadReconnectData(): {
    roomCode: number,
    playerId: number,
    lastSaveTime: number,
} | null {
    let data = localStorage.getItem("reconnectData");
    // localStorage.removeItem("reconnectData");
    if (data) {
        return JSON.parse(data);
    }
    return null;
}




export function saveSettings(settings: Partial<Settings>) {
    localStorage.setItem("settings", JSON.stringify({
        ...loadSettings(),
        ...settings,
    }));
}
export function loadSettings(): Partial<Settings>{
    const data = localStorage.getItem("settings");
    if (data !== null) {
        return JSON.parse(data);
    }
    return {};
}



export function defaultGameModes(): SavedGameModes {
    return DEFAULT_GAME_MODES as SavedGameModes;
}

export type SavedGameModes = GameMode[];

export type GameMode = {
    name: string,
    // A mapping from number-of-players to game mode data
    data: Record<number, GameModeData>
};

export type GameModeData = {
    roleList: RoleList,
    phaseTimes: PhaseTimes,
    disabledRoles: Role[],
}

export function saveGameModes(roleList: SavedGameModes) {
    localStorage.setItem("savedGameModes", JSON.stringify(roleList));
}
export function loadGameModes(): NonNullable<unknown> | SavedGameModes | null {
    const data = localStorage.getItem("savedGameModes");
    if (data !== null) {
        try {
            return JSON.parse(data);
        } catch {
            return null;
        }
    }
    return defaultGameModes();
}
export function deleteGameModes() {
    localStorage.removeItem("savedGameModes");
}