import { createContext, useContext, useState } from "react";
import { ClientConnection, LobbyClientID, LobbyClientType, ModifierType, PhaseTimes } from "../../game/gameState.d";
import ListMap from "../../ListMap";
import { RoleList } from "../../game/roleListState.d";
import { Role } from "../../game/roleState.d";
import { ChatMessage } from "../../components/ChatMessage";


export function useLobbyStateContext(){
    const [lobbyState, setLobbyState] = useState<LobbyState>(createLobbyState());
    
    const incomingPacketFuck = useContext();
    whenever message then setLobbyState

    return lobbyState;
}

const LobbyStateContext = createContext<LobbyState | undefined>(undefined)
export { LobbyStateContext }

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
export type LobbyClient = {
    ready: "host" | "ready" | "notReady",
    connection: ClientConnection,
    clientType: LobbyClientType
}
function createLobbyState(): LobbyState {
    return {
        type: "lobby",
        roomCode: 0,
        lobbyName: "Mafia Lobby",

        myId: null,

        roleList: [],
        phaseTimes: defaultPhaseTimes(),
        enabledRoles: [],
        enabledModifiers: [],

        players: new ListMap<LobbyClientID, LobbyClient>(),
        chatMessages: [],
    }
}
function defaultPhaseTimes(): PhaseTimes {
    return {
        briefing: 45,
        obituary: 60,
        discussion: 120,
        nomination: 120,
        testimony: 30,
        judgement: 60,
        finalWords: 30,
        dusk: 30,
        night: 60,
    }
}