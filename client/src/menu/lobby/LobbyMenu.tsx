import React from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import LobbyPhaseTimePane from "./LobbyPhaseTimePane";
import LobbyRolePane from "./LobbyRolePane";
import "./lobbyMenu.css";
import translate from "../../game/lang";

export function create() {
    return <div className="lm">
        <header>
            <h1>
                {translate("menu.lobby.title", GAME_MANAGER.roomCode!)}
            </h1>
            <button onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
                {translate("menu.lobby.button.start")}
            </button>
        </header>

        <main>
            <div className="left">
                <LobbyPlayerList/>
            </div>
            <div className="right">
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
            </div>

        </main>
    </div>
}
