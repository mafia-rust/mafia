import { ReactElement } from "react";
import { LobbyPreviewData } from "../../game/packet";
import React from "react";
import { joinGame } from "./PlayMenu";
import translate from "../../game/lang";
import "./playMenuJoinPopup.css";

export default function PlayMenuJoinPopup(props: {
    roomCode: number,
    lobbyData: LobbyPreviewData
}): ReactElement {

    return <div className="play-menu-join-popup">
        <button onClick={() => joinGame(props.roomCode)}>{translate("menu.play.button.spectate")}</button>
        
        <div className="rejoinColumn">
            {props.lobbyData.players.map((player, j)=>{
                return <button key={j} onClick={()=>{
                    joinGame(props.roomCode, player[0]);
                }}>{player[1]}</button>
            })}
        </div>
    </div>
}