import { createContext, useContext, useState } from "react";
import GameState, { PlayerGameState } from "../../game/gameState.d";


export function useGameStateContext(initial: GameState): GameState{
    const [gameState, setGameState] = useState<GameState>(initial);
    
    const incomingPacketFuck = useContext();
    whenever message then setGameState

    return gameState;
}

export function getMyPlayerState(gameState?: GameState): PlayerGameState | undefined {
    if(gameState === undefined || gameState.clientState.type==="spectator"){
        return undefined;
    }else{
        return gameState.clientState;
    }
}

const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }