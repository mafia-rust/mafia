import { createContext, useContext, useEffect, useState } from "react";
import { WebsocketContext } from "../WebsocketContext";
import { ToClientPacket } from "../../packet";
import { createGameState } from "../../stateContext/stateType/gameState";

export function useGameStateContext(): GameState{
    const [gameState, setGameState] = useState<GameState>(createGameState());

    setGameState(gameState => {
        return {
            ...gameState,
            updateChatFilter: (filter: number | null) => {
                if(gameState.clientState.type === "player"){
                    gameState.clientState.chatFilter = filter===null?null:{
                        type: "playerNameInMessage",
                        player: filter
                    };
                }
            },
            setPrependWhisperFunction: (f) => {
                gameState.prependWhisper = f;
            },
        }
    })

    return gameState;
}


