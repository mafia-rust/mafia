import React, { ReactElement, useEffect, useState } from "react";
import translate from "../../game/lang";
import Anchor from "../Anchor";
import GAME_MANAGER from "../..";
import LoadingScreen from "../LoadingScreen";
import "./playMenu.css";
import { StateListener } from "../../game/gameManager.d";
import { LobbyPreviewData } from "../../game/packet";
import LobbyMenu from "../lobby/LobbyMenu";
import PlayMenuJoinPopup from "./PlayMenuJoinPopup";

export default function PlayMenu(): ReactElement {
    
    useEffect(() => {
        GAME_MANAGER.sendLobbyListRequest();
        
        const autoRefresh = setInterval(() => {GAME_MANAGER.sendLobbyListRequest()}, 2500);
        return () => clearInterval(autoRefresh);
    })

    return <div className="play-menu">
        <div className="play-menu-browser graveyard-menu-colors">
            <header>
                <h1>
                    {translate("menu.play.title")}
                </h1>
                <div>
                    <button onClick={()=>{hostGame()}}>
                        {translate("menu.play.button.host")}
                    </button>
                    <button onClick={()=>{GAME_MANAGER.sendLobbyListRequest()}}>
                        {translate("refresh")}
                    </button>
                </div>
            </header>
            <div className="play-menu-center">
                <PlayMenuTable />
            </div>
            <PlayMenuFooter />
        </div>
    </div>
}

function PlayMenuFooter(): ReactElement {
    const [roomCode, setRoomCode] = useState<number | undefined>(undefined);
    const [playerId, setPlayerID] = useState<number | undefined>(undefined);

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
                        joinGame(roomCode);
                    }
                }}
            />
        </div>
        <div>
            <label>{translate("menu.play.field.playerId")}</label>
            <input type="text" value={playerId} 
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
                        joinGame(roomCode, playerId);
                    }
                }}
            />
        </div>
        <button onClick={()=>{
            joinGame(roomCode, playerId)
        }}>
            {translate("menu.play.button.join")}
        </button>
    </footer>
}

export async function joinGame(roomCode?: number, playerId?: number): Promise<boolean> {
    if (roomCode === undefined) return false;

    Anchor.setContent(<LoadingScreen type="join"/>);

    let success: boolean;
    if (playerId === undefined) {
        success = await GAME_MANAGER.sendJoinPacket(roomCode);
    } else {
        success = await GAME_MANAGER.sendRejoinPacket(roomCode, playerId);
    }

    if (!success) {
        Anchor.setContent(<PlayMenu/>);
    }

    return success;
}

async function hostGame() {
    Anchor.setContent(<LoadingScreen type="host"/>);
    if (await GAME_MANAGER.sendHostPacket()) {
        Anchor.setContent(<LobbyMenu/>)
    } else {
        Anchor.setContent(<PlayMenu/>)
    }
}

type LobbyMap = Map<number, LobbyPreviewData>;

function PlayMenuTable(): ReactElement {
    const [lobbies, setLobbies] = useState<LobbyMap>(new Map());

    useEffect(() => {
        const listener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "outsideLobby" && type === "lobbyList") {
                setLobbies(GAME_MANAGER.state.lobbies);
            }
        }
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    });

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
                        <button onClick={() => {
                            if(lobby.inGame){
                                Anchor.setCoverCard(<PlayMenuJoinPopup 
                                    roomCode={roomCode}
                                    lobbyData={lobby}
                                />);
                            }else{
                                joinGame(roomCode);
                            }
                        }}>{translate("menu.play.button.join")}</button>
                    </td>
                    <td>{lobby.name}</td>
                    <td>
                        <div className="play-menu-lobby-player-list">
                            {lobby.players.map((player, j)=>{
                                return <button key={j} onClick={()=>{
                                    joinGame(roomCode, player[0]);
                                }}>{player[1]}</button>
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