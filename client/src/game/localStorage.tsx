import { PhaseTimes } from "./gameState.d";
import { RoleList, RoleOutline } from "./roleListState.d";

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




export function saveSettings(volume: number) {
    localStorage.setItem("settings", JSON.stringify({
        "volume": volume,
    }));
}
export function loadSettings(): { volume: number } | null{
    let data = localStorage.getItem("settings");
    if (data) {
        return JSON.parse(data);
    }
    return null;
}





export type SavedGameModes = Map<string, {
    roleList: RoleList,
    phaseTimes: PhaseTimes,
    excludedRoles: RoleOutline | null
}>;

export function saveGameModes(roleList: SavedGameModes) {
    localStorage.setItem("savedGameModes", JSON.stringify(roleList, replacer));
}
export function loadGameModes(): SavedGameModes | null{
    let data = localStorage.getItem("savedGameModes");
    if (data) {
        return JSON.parse(data, reviver);
    }
    return null;
}



function replacer(key: any, value: any) {
    if(value instanceof Map) {
        return {
            dataType: 'Map',
            value: Array.from(value.entries()), // or with spread: value: [...value]
        };
    } else {
      return value;
    }
}
function reviver(key: any, value: any) {
    if(typeof value === 'object' && value !== null) {
        if (value.dataType === 'Map') {
            return new Map(value.value);
        }
    }
    return value;
}