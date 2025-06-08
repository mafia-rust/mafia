import { createContext, useContext, useState } from "react";
import { GameState } from "./stateType/gameState";
import { LobbyState } from "./stateType/lobbyState";
import { State } from "./state";

export const StateContext = createContext<State | undefined>(undefined);
export function useStateContext(): State {
    const [state, setState] = useState({type: "disconnected" as const});
    return state;
}


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
    if(stateCtx.type==="lobby"){
        return stateCtx;
    }
    return undefined;
}
export function useContextGameState(): State | undefined{
    const stateCtx = useContext(StateContext)!;
    if(stateCtx.type==="game"){
        return stateCtx;
    }
    return undefined;
}