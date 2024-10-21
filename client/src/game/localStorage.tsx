import DEFAULT_GAME_MODES from "../resources/defaultGameModes.json";
import { GameModeStorage } from "../components/gameModeSettings/gameMode";
import { Language } from "./lang";
import { Role } from "./roleState.d";
import parseFromJson from "../components/gameModeSettings/gameMode/dataFixer";

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
    let dataJSON = localStorage.getItem("reconnectData");
    
    if (dataJSON) {
        let reconnectData = JSON.parse(dataJSON);
    
        // Make sure it isn't expired
        const HOUR_IN_SECONDS = 3_600_000;
        if (reconnectData.lastSaveTime < Date.now() - HOUR_IN_SECONDS) {
            deleteReconnectData();
            return null
        }

        return reconnectData;
    }

    return null;
}



export type Settings = {
    volume: number;
    defaultName: string | null;
    language: Language;
    roleSpecificMenus: Role[] // RoleSpecificMenuType=standalone for all listed roles, otherwise it should be playerlist
};

export type RoleSpecificMenuType = "playerList" | "standalone";



export function loadSettingsParsed(): Settings {
    const result = parseFromJson("Settings", loadSettings());
    if(result.type === "failure") {
        return DEFAULT_SETTINGS;
    }else{
        return result.value;
    }
}

export function loadSettings(): unknown {
    const data = localStorage.getItem("settings");
    if (data !== null) {
        try {
            return JSON.parse(data);
        } catch {
            return null;
        }
    }
    return DEFAULT_SETTINGS;
}
export function saveSettings(settings: Partial<Settings>) {
    const currentSettings = parseFromJson("Settings", loadSettings());

    if(currentSettings.type === "failure") {
        localStorage.setItem("settings", JSON.stringify({
            ...DEFAULT_SETTINGS,
            ...settings,
        }));
    }else{
        localStorage.setItem("settings", JSON.stringify({
            ...currentSettings,
            ...settings,
        }));
    }
}


export function defaultGameModes(): unknown {
    // Typescript is a Division One tweaker
    return DEFAULT_GAME_MODES;
}

export function saveGameModes(gameModes: GameModeStorage) {
    localStorage.setItem("savedGameModes", JSON.stringify(gameModes));
}
export function loadGameModes(): unknown {
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
    defaultName: null,
    roleSpecificMenus: []
};