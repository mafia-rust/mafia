import { createContext, useContext, useState } from "react";
import GameState, { LobbyState, PlayerGameState } from "../../game/gameState.d";


export function useGameStateContext(initial: GameState): GameState{
    const [gameState, setGameState] = useState<GameState>(initial);
    
    const incomingPacketFuck = useContext();
    whenever message then setGameState

    return gameState;
}

export function usePlayerState(gameState?: GameState): PlayerGameState | undefined {
    if(gameState === undefined || gameState.clientState.type==="spectator"){
        return undefined;
    }else{
        return gameState.clientState;
    }
}
export function usePlayerNames(state?: GameState | LobbyState): string[] | undefined {
    if(state===undefined){
        return undefined
    }
    if(state.stateType === "game"){
        return state.players.map((p)=>p.name)
    }
    return state.players.values()
        .filter((c)=>c.clientType.type==="player")
        //thanks typescript very cool
        .map((c)=>c.clientType.type==="player"?c.clientType.name:undefined) as string[]

}

const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }