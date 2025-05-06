import { createContext, useContext, useState } from "react";
import GameState from "../../game/gameState.d";


function useGameStateContext(initial: GameState){
    const [gameState, setGameState] = useState<GameState>(initial);
    
    const incomingPacketFuck = useContext();
    whenever message then setGameState
}

const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }