import React, { createContext, ReactElement, useCallback, useContext, useEffect, useState } from "react";
import translate from "../../game/lang";
import LoadingScreen from "../LoadingScreen";
import "./playMenu.css";
import { LobbyPreviewData, ToClientPacket } from "../../game/packet";
import LobbyMenu from "../lobby/LobbyMenu";
import PlayMenuJoinPopup from "./PlayMenuJoinPopup";
import { Button } from "../../components/Button";
import { AnchorContext } from "../AnchorContext";
import { WebsocketContext } from "../WebsocketContext";


type LobbyMap = Map<number, LobbyPreviewData>;
type PlayMenuContextType = {
    lobbies: LobbyMap,
    setLobbies: (state: LobbyMap)=>void
}
const PlayMenuContext = createContext<PlayMenuContextType | undefined>(undefined);

function usePlayMenuContext(): PlayMenuContextType {
    const [lobbies, setLobbies] = useState(new Map<number, LobbyPreviewData>());
    return {
        lobbies, setLobbies
    };
}

function playMenuMessageListener(packet: ToClientPacket, playMenuContext: PlayMenuContextType){
    const {setLobbies} = playMenuContext;

    switch(packet.type){
        case "lobbyList":{
            const lobbies = new Map();

            for(let [lobbyId, lobbyData] of Object.entries(packet.lobbies))
                lobbies.set(Number.parseInt(lobbyId), lobbyData);

            setLobbies(lobbies);
        }
        break;
    }
}


export default function PlayMenu(): ReactElement {
    const { setContent: setAnchorContent } = useContext(AnchorContext)!;
    const {lastMessageRecieved, sendLobbyListRequest, sendJoinPacket, sendRejoinPacket, sendHostPacket} = useContext(WebsocketContext)!;
    const playMenuContext = usePlayMenuContext();

    useEffect(()=>{
        if(lastMessageRecieved){
            playMenuMessageListener(lastMessageRecieved, playMenuContext);
        }
    }, [lastMessageRecieved]);
    
    useEffect(() => {
        sendLobbyListRequest();
        
        const FIVE_SECONDS = 5000
        const autoRefresh = setInterval(sendLobbyListRequest, FIVE_SECONDS);
        return () => clearInterval(autoRefresh);
    })

    const joinGame = useCallback(
        async (roomCode?: number, playerId?: number): Promise<boolean> => {
            if (roomCode === undefined) return false;
        
            setAnchorContent(<LoadingScreen type="join"/>);
        
            let success: boolean;
            if (playerId === undefined) {
                success = await sendJoinPacket(roomCode);
            } else {
                success = await sendRejoinPacket(roomCode, playerId);
            }
        
            if (!success) {
                setAnchorContent(<PlayMenu/>);
            }
        
            return success;
        },
        [setAnchorContent]
    );
    

    return <PlayMenuContext.Provider value={playMenuContext}>
        <div className="play-menu">
            <div className="play-menu-browser graveyard-menu-colors">
                <header>
                    <h2>
                        {translate("menu.play.title")}
                    </h2>
                    <div>
                        <Button onClick={async () => {
                            setAnchorContent(<LoadingScreen type="host"/>);
                            if (await sendHostPacket()) {
                                setAnchorContent(<LobbyMenu/>)
                            } else {
                                setAnchorContent(<PlayMenu/>)
                            }
                        }}>
                            {translate("menu.play.button.host")}
                        </Button>
                        <Button onClick={()=>{sendLobbyListRequest()}}>
                            {translate("refresh")}
                        </Button>
                    </div>
                </header>
                <div className="play-menu-center">
                    <PlayMenuTable joinGame={joinGame}/>
                </div>
                <PlayMenuFooter joinGame={joinGame}/>
            </div>
        </div>
    </PlayMenuContext.Provider>
}

function PlayMenuFooter(props: Readonly<{
    joinGame: (roomCode?: number, playerId?: number) => Promise<boolean>
}>): ReactElement {
    const [roomCode, setRoomCode] = useState<number | undefined>(undefined);
    const [playerID, setPlayerID] = useState<number | undefined>(undefined);

    return <footer>
        <div>
            <label>{translate("menu.play.field.roomCode")}</label>
            <input type="text" value={roomCode?.toString(18)} 
                onChange={(e)=>{
                    const value = e.target.value;
                    if (value === "") {
                        setRoomCode(undefined);
                    } else {
                        try {
                            const code = parseInt(value, 18);
                            if (!isNaN(code)) {
                                setRoomCode(code)
                            }
                        } catch (_) {}
                    }}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter') {
                        props.joinGame(roomCode);
                    }
                }}
            />
        </div>
        <div>
            <label>{translate("menu.play.field.playerId")}</label>
            <input type="text" value={playerID} 
                onChange={(e)=>{
                    const value = e.target.value;
                    if (value === "") {
                        setPlayerID(undefined);
                    } else {
                        try {
                            const id = parseInt(value);
                            if (!isNaN(id) && id < 256) {
                                setPlayerID(id)
                            }
                        } catch (_) {}
                    }
                }}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter') {
                        props.joinGame(roomCode, playerID);
                    }
                }}
            />
        </div>
        <button onClick={()=>{
            props.joinGame(roomCode, playerID)
        }}>
            {translate("menu.play.button.join")}
        </button>
    </footer>
}

function PlayMenuTable(props: Readonly<{
    joinGame: (roomCode?: number, playerId?: number) => Promise<boolean>
}>): ReactElement {
    const { setCoverCard } = useContext(AnchorContext)!;
    const { lobbies } = useContext(PlayMenuContext)!;

    return <table className="play-menu-table">
        <thead>
            <tr>
                <th></th>
                <th>{translate("menu.play.field.name")}</th>
                <th>{translate("players")}</th>
            </tr>
        </thead>
        <tbody>
            {Array.from(lobbies.entries()).map((entry)=>{
                const roomCode = entry[0];
                const lobby: LobbyPreviewData = entry[1];

                return <tr key={roomCode}>
                    <td>
                        <Button onClick={() => {
                            if(lobby.inGame){
                                setCoverCard(<PlayMenuJoinPopup 
                                    roomCode={roomCode}
                                    lobbyData={lobby}
                                    joinGame={props.joinGame}
                                />);
                            }else{
                                props.joinGame(roomCode);
                            }
                        }}>{translate("menu.play.button.join")}</Button>
                    </td>
                    <td>{lobby.name}</td>
                    <td>
                        <div className="play-menu-lobby-player-list">
                            {lobby.players.map((player)=>{
                                return <Button key={player[1]} onClick={()=>{
                                    props.joinGame(roomCode, player[0]);
                                }}>{player[1]}</Button>
                            })}
                        </div>
                    </td>
                </tr>;
            })}
        </tbody>
        <tfoot>
            {new Array(100).fill(0).map((_, i) => {
                return <tr key={i}>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
            })}
        </tfoot>
    </table>
}