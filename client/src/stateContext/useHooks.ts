import { useContext } from "react";
import { GameState, PlayerGameState } from "./stateType/gameState";
import { LobbyState } from "./stateType/lobbyState";
import { StateContext } from "./StateContext";
import { State } from "./state";


export function useLobbyOrGameState<T = GameState|LobbyState>(
    map: (state: GameState|LobbyState)=>T = (state)=>state as T
):T | undefined{
    const state = useContext(StateContext)!;
    if(state.type==="game"||state.type==="lobby"){
        return map(state);
    }
    return undefined;
}
export function useContextLobbyState(): State & {type: "lobby"} | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.state.type==="lobby"){
        return stateCtx;
    }
    return undefined;
}
export function useContextGameState(): State & {type: "game"} | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.state.type==="game"){
        return stateCtx;
    }
    return undefined;
}


export function usePlayerState(): PlayerGameState | undefined {
    const gameState = useContext(StateContext);
    if(gameState?.type !== "game"){return undefined};
    if(gameState === undefined){return undefined};
    const { clientState } = gameState

    if (clientState.type === "player") {
        return clientState
    } else {
        return undefined
    }
}
export function usePlayerNames(): string[] | undefined {
    const state = useContext(StateContext);
    if(state===undefined){return undefined}
    if(state.type === "game"){
        return state.players.map((p)=>p.name)
    }else if(state.type === "lobby"){
        return state.players.values()
            .filter((c)=>c.clientType.type==="player")
            //thanks typescript very cool
            .map((c)=>c.clientType.type==="player"?c.clientType.name:undefined) as string[]
    }
}