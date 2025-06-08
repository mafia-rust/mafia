import { createContext, useContext, useEffect, useState } from "react";
import { ChatGroup, GameClient, InsiderGroup, LobbyClientID, ModifierType, PhaseState, PhaseTimes, PlayerIndex, Tag, Verdict } from "../../game/gameState.d";
import { Role, RoleState } from "../../game/roleState.d";
import { ChatMessage } from "../../components/ChatMessage";
import { Grave } from "../../game/graveState";
import { RoleList } from "../../stateContext/stateType/roleListState";
import ListMap, { ListMapData } from "../../ListMap";
import { ChatFilter } from "./gameScreenContent/ChatMenu";
import { ControllerID, SavedController } from "../../game/abilityInput";
import { defaultPhaseTimes } from "../../game/localStorage";
import { LobbyState } from "../lobby/LobbyContext";
import { WebsocketContext } from "../WebsocketContext";
import { ToClientPacket } from "../../packet";

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

    const websocketContext = useContext(WebsocketContext)!;

    useEffect(()=>{
        if(websocketContext.lastMessageRecieved){
            gameStateMessageListener(websocketContext.lastMessageRecieved, gameState);
            //for now
            setGameState(gameState);
        }
    }, [websocketContext.lastMessageRecieved]);

    return gameState;
}

function gameStateMessageListener(packet: ToClientPacket, gameState: GameState){
    
}

export function usePlayerState(): PlayerGameState | undefined {
    const gameState = useContext(GameStateContext);
    if(gameState === undefined){return undefined};
    const { clientState } = gameState

    if (clientState.type === "player") {
        return clientState
    } else {
        return undefined
    }
}
export function usePlayerNames(state?: GameState | LobbyState): string[] | undefined {
    if(state===undefined){
        return undefined;
    }
    if(state.type === "game"){
        return state.players.map((p)=>p.name)
    }
    return state.players.values()
        .filter((c)=>c.clientType.type==="player")
        //thanks typescript very cool
        .map((c)=>c.clientType.type==="player"?c.clientType.name:undefined) as string[]
}
const GameStateContext = createContext<GameState | undefined>(undefined)
export { GameStateContext }


