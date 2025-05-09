import { createContext, useContext, useState } from "react";
import { LobbyState } from "../../game/gameState.d";


export function useLobbyStateContext(initial: LobbyState){
    const [lobbyState, setLobbyState] = useState<LobbyState>(initial);
    
    const incomingPacketFuck = useContext();
    whenever message then setLobbyState

    return lobbyState;
}

const LobbyStateContext = createContext<LobbyState | undefined>(undefined)
export { LobbyStateContext }