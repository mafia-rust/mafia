import { createContext, useContext, useState } from "react";
import GameState from "../../game/gameState.d";


export function useGameStateContext(initial: GameState): GameState{
    const [gameState, setGameState] = useState<GameState>(initial);
    
    const incomingPacketFuck = useContext();
    whenever message then setGameState

    return gameState;
}

const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }