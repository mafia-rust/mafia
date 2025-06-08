import { ChatMessage } from "../../components/ChatMessage"
import { LobbyClientID, ModifierType, PhaseTimes } from "../game/gameState.d"
import { RoleList } from "./roleListState"
import { Role } from "../game/roleState.d"
import ListMap from "../../ListMap"

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
function createLobbyState(roomCode: number, myId: number): LobbyState {
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