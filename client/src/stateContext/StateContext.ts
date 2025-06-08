import { createContext, useContext, useState } from "react";
import { GameState, PlayerGameState } from "./stateType/gameState";
import { LobbyState } from "./stateType/lobbyState";
import { State } from "./state";

export const StateContext = createContext<State | undefined>(undefined);
export function useStateContext(): State {
    const [state, setState] = useState({type: "disconnected" as const});
    return state;
}