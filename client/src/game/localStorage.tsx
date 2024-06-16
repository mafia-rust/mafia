import { Settings } from "../menu/Settings";
import DEFAULT_GAME_MODES from "../resources/defaultGameModes.json";
import { GameModeStorage } from "../components/gameModeSettings/gameMode";

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
            localStorage.clear();
            return null;
        }
    }
    return defaultGameModes();
}
export function deleteGameModes() {
    localStorage.removeItem("savedGameModes");
}