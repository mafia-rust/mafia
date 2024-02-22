import { RoleList } from "./roleListState.d";

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





export type SavedRoleLists = Map<string, RoleList>;

export function saveRoleLists(roleList: SavedRoleLists) {
    localStorage.setItem("savedRoleLists", JSON.stringify(roleList));
}
export function loadRoleLists(): SavedRoleLists | null{
    let data = localStorage.getItem("savedRoleLists");
    if (data) {
        return JSON.parse(data);
    }
    return null;
}