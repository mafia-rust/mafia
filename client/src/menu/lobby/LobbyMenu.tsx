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
            <input type="text" disabled value={GAME_MANAGER.roomCode!} id="lobbyRoomCode" />
            <button onClick={() => {
                let code = new URL(window.location.href);
                code.searchParams.set("code", GAME_MANAGER.roomCode!);
                navigator.clipboard.writeText(code.toString());
            }}>
                {translate("menu.lobby.button.copy")}
            </button>
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
