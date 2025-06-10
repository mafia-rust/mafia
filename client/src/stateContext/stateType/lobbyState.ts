import { ChatMessage } from "../../components/ChatMessage"
import { RoleList } from "./roleListState"
import ListMap from "../../ListMap"
import { ClientConnection, defaultPhaseTimes, LobbyClientID, LobbyClientType, PhaseTimes } from "./otherState"
import { ModifierType } from "./modifiersState"
import { Role } from "./roleState"

export type LobbyState = {
    type: "lobby"
    roomCode: number,
    lobbyName: string,

    myId: number | null,

    roleList: RoleList,
    phaseTimes: PhaseTimes,
    enabledRoles: Role[],
    enabledModifiers: ModifierType[],

    players: ListMap<LobbyClientID, LobbyClient>,
    chatMessages: ChatMessage[],
}
export function createLobbyState(roomCode: number, myId: number): LobbyState {
    return {
        type: "lobby",
        roomCode,
        lobbyName: "Mafia Lobby",

        myId,

        roleList: [],
        phaseTimes: defaultPhaseTimes(),
        enabledRoles: [],
        enabledModifiers: [],

        players: new ListMap<LobbyClientID, LobbyClient>(),
        chatMessages: [],
    }
}


export type LobbyClient = {
    ready: "host" | "ready" | "notReady",
    connection: ClientConnection,
    clientType: LobbyClientType
}