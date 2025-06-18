import { useContext } from "react";
import { GameState, PlayerGameState } from "./stateType/gameState";
import { LobbyState } from "./stateType/lobbyState";
import { StateContext } from "./StateContext";
import { State } from "./state";


export function useLobbyOrGameState<T = GameState|LobbyState>(
    map: (state: GameState|LobbyState)=>T = (state)=>state as T
):T | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.state.type==="game"||stateCtx.state.type==="lobby"){
        return map(stateCtx.state);
    }
    return undefined;
}
export function useContextLobbyState(): State & {type: "lobby"} | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.state.type==="lobby"){
        return stateCtx.state;
    }
    return undefined;
}
export function useContextGameState(): State & {type: "game"} | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.state.type==="game"){
        return stateCtx.state;
    }
    return undefined;
}


export function usePlayerState(): PlayerGameState | undefined {
    const stateCtx = useContext(StateContext);
    if(stateCtx?.state.type !== "game"){return undefined};
    if(stateCtx === undefined){return undefined};
    const { clientState } = stateCtx.state

    if (clientState.type === "player") {
        return clientState
    } else {
        return undefined
    }
}
export function usePlayerNames(): string[] | undefined {
    const stateCtx = useContext(StateContext);
    if(stateCtx===undefined){return undefined}
    if(stateCtx.state.type === "game"){
        return stateCtx.state.players.map((p)=>p.name)
    }else if(stateCtx.state.type === "lobby"){
        return stateCtx.state.players.values()
            .filter((c)=>c.clientType.type==="player")
            //thanks typescript very cool
            .map((c)=>c.clientType.type==="player"?c.clientType.name:undefined) as string[]
    }
}