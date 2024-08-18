import DEFAULT_GAME_MODES from "../resources/defaultGameModes.json";
import { GameModeStorage } from "../components/gameModeSettings/gameMode";
import { Language } from "./lang";
import { Role } from "./roleState.d";
import { getAllRoles } from "./roleListState.d";

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



export type Settings = {
    volume: number;
    language: Language;
    roleSpecificMenus: Record<Role, RoleSpecificMenuType>
};

export type RoleSpecificMenuType = "playerList" | "standalone";


export function saveSettings(settings: Partial<Settings>) {
    localStorage.setItem("settings", JSON.stringify({
        ...loadSettings(),
        ...settings,
    }));
}

export function loadSettings(): Settings {
    const data = localStorage.getItem("settings");
    if (data !== null) {
        return {...DEFAULT_SETTINGS, ...JSON.parse(data)};
    }
    return DEFAULT_SETTINGS;
}



export function defaultGameModes(): GameModeStorage {
    // Typescript is a Division One tweaker
    return DEFAULT_GAME_MODES as unknown as GameModeStorage;
}

export function saveGameModes(roleList: GameModeStorage) {
    localStorage.setItem("savedGameModes", JSON.stringify(roleList));
}
export function loadGameModes(): NonNullable<unknown> | null {
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


export const DEFAULT_SETTINGS: Readonly<Settings> = {
    volume: 0.5,
    language: "en_us",
    roleSpecificMenus: Object.fromEntries(getAllRoles().map(role => [role, "playerList"])) as Record<Role, "playerList">
};