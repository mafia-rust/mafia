import { createContext, useContext, useState } from "react";
import { ClientConnection, LobbyClientID, LobbyClientType, ModifierType, PhaseTimes } from "../../game/gameState.d";
import ListMap from "../../ListMap";
import { RoleList } from "../../game/roleListState.d";
import { Role } from "../../game/roleState.d";
import { ChatMessage } from "../../components/ChatMessage";
import { defaultPhaseTimes } from "../../game/localStorage";
import GameState, { GameStateContext } from "../game/GameStateContext";


export function useLobbyStateContext(roomCode: number, myId: number){
    const [lobbyState, setLobbyState] = useState<LobbyState>(createLobbyState(roomCode, myId));
    
    // const incomingPacketFuck = useContext();
    // whenever message then setLobbyState

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
export function useLobbyOrGameState<T = GameState|LobbyState>(
    map: (state: GameState|LobbyState)=>T = (state)=>state as T
):T | undefined{
    const gameState = useContext(GameStateContext);
    const lobbyState = useContext(LobbyStateContext);

    if(gameState!==undefined){
        return map(gameState);
    }else if(lobbyState!==undefined){
        return map(lobbyState);
    }else{
        return undefined;
    }
}