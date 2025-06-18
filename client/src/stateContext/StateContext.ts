import { createContext, useContext, useState } from "react";
import { State } from "./state";
import { createGameState } from "./stateType/gameState";
import { createLobbyState } from "./stateType/lobbyState";
import { createGameBrowserState, LobbyPreviewData } from "./stateType/gameBrowserState";
import AudioController from "../menu/AudioController";
import ListMap from "../ListMap";
import { WebsocketContext } from "../menu/WebsocketContext";

export type StateContext = {
    state: State,
    setDisconnected(): void,
    setGameBrowser(): void,
    setLobby(roomCode: number, myId: number): void,
    setGame(spectator: boolean): void,
}


export const StateContext = createContext<StateContext | undefined>(undefined);
export function useStateContext(): StateContext {
    const websocketCtx = useContext(WebsocketContext)!;

    const [state, setState] = useState<State>({type: "disconnected" as const});
    
    let setDisconnected = ()=>{
        websocketCtx.close();
        setState({type: "disconnected" as const});
    };
    let setGameBrowser = ()=>{
        AudioController.clearQueue();
        AudioController.pauseQueue();
        
        websocketCtx.open().then(()=>{
            setState(()=>{
                return createGameBrowserState();
            })
        });
    };
    let setLobby = (roomCode: number, myId: number)=>{
        setState((state)=>{
            if(state.type==="game"){
                let {roomCode, lobbyName, roleList, phaseTimes, enabledRoles} = state;
                let newState = createLobbyState(roomCode, myId);
                newState.roomCode = roomCode;
                newState.lobbyName = lobbyName;
                newState.roleList = roleList;
                newState.phaseTimes = phaseTimes;
                newState.enabledRoles = enabledRoles;
                return newState;
            }else{
                return createLobbyState(roomCode, myId);
            }
        });
    };
    let setGame = (spectator: boolean)=>{

        AudioController.clearQueue();
        AudioController.unpauseQueue();

        setState(()=>{
            if(state.type==="lobby"){

                let {players, myId, roomCode, lobbyName, roleList, phaseTimes, enabledRoles} = state;
                let isHost = players.get(myId!)?.ready === "host";
                let newState = createGameState(spectator);
                newState.roomCode = roomCode;
                newState.lobbyName = lobbyName;
                newState.roleList = roleList;
                newState.phaseTimes = phaseTimes;
                newState.enabledRoles = enabledRoles;
                
                if (isHost) {
                    newState.host = {
                        clients: new ListMap()
                    };
                }

                return newState;
            }else{
                return createGameState(spectator);
            }
        });

            
    };


    
    return {
        state, setDisconnected, setGame, setLobby, setGameBrowser
    };
}