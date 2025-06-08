import { LobbyClientID } from "./otherState"

export type Disconnected = {
    type: "disconnected"
}

export type GameBrowserState = {
    type: "gameBrowser",

    selectedRoomCode: string | null,
    lobbies: Map<number, LobbyPreviewData>,
}

export type LobbyPreviewData = {
    name: string,
    inGame : boolean,
    players: [LobbyClientID, string][]
}